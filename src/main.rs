/*
:TODO:
- Multiple shapes - better/more efficient storage/organisation.
- Shading model.
- Lights.
- Efficient scene organisation.
- Multi-threaded rendering.
- Spectral path tracing (single scalar per ray? Randomly pick a wavelength per bounce?)
*/

extern crate cgmath;
extern crate image;
extern crate rand;

use cgmath::*;
use image::ColorType;
use image::png::PNGEncoder;
use rand::{random};
use std::f32;
use std::fs::File;

pub mod materials;
pub mod shapes;
pub mod util;

use materials::{dialectric, lambertian, metal};
use shapes::{sphere};
use util::{maths, random};

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fov: Deg<f32>,
    near: f32,
    far: f32,
    aperture: f32,
    focus_distance: f32,
}

fn write_png_rgb8(filename: &str, pixels: &[u8], dimensions: (u32, u32))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, dimensions.0, dimensions.1, ColorType::RGB(8))?;
    Ok(())
}

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>
}

struct Interval {
    min: f32,
    max: f32
}

struct Hit<'a> {
    distance: f32,
    location: Point3<f32>,
    normal: Vector3<f32>,
    material: &'a (Scatterable + 'a), // :TODO: Better undestand lifetime use here
}

trait Hitable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}


struct Scatter {
    ray: Ray,
    attenuation: Vector3<f32>,
}

trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

fn hit<'a>(shapes: &'a Vec<Box<Hitable>>, ray: &Ray, interval: &Interval) -> Option<Hit<'a>> {
    let mut hit_result: Option<Hit> = None;
    let mut closest = interval.max;
    for shape in shapes
    {
        if let Some(hit) = shape.hit(ray, &Interval { min: interval.min, max: closest }) {
            closest = hit.distance;
            hit_result = Some(hit);
        }
    }
    return hit_result;
}

fn trace(shapes: &Vec<Box<Hitable>>, ray: &Ray, depth: u32) -> Vector3<f32> {
    let hit = hit(shapes, ray, &Interval { min: 0.001, max: f32::MAX });
    let colour = match hit {
        None => {
            let t = 0.5 * (ray.direction.y + 1.0);
            ((1.0 - t) * vec3(1., 1., 1.)) + (t * vec3(0.5, 0.7, 1.0))
        },
        Some(hit) => {
            if depth < 50 {
                let scatter_result = hit.material.scatter(ray, &hit);
                let colour = match scatter_result {
                    None => vec3(0., 0., 0.),
                    Some(scatter_result) => scatter_result.attenuation.mul_element_wise(trace(shapes, &scatter_result.ray, depth + 1))
                };
                colour
            } else {
                vec3(0., 0., 0.)
            }
        }
    };
    colour
}

fn main() {
    // :TODO:
    // - Preview output in window.
    // - Command line args parsing (output to file/window).
    // - 

    let image_width = 640;
    let image_height = 400;
    let image_aspect = image_width as f32 / image_height as f32;
    let num_samples = 50;

    let cam_eye = Point3::new(0., 0., 0.75);
    let cam_target = Point3::new(0., 0., -1.);
    let camera = Camera {
        eye: cam_eye,
        target: cam_target,
        up: vec3(0., 1., 0.),
        fov: Deg(60.),
        near: 0.01,
        far: 100.,
        aperture: 0.4,
        focus_distance: (cam_target - cam_eye).magnitude(),
    };

    let view_matrix = Matrix4::look_at(camera.eye, camera.target, camera.up);
    let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
    let view_projection_matrix = projection_matrix * view_matrix;
    let inv_view_projection_matrix = view_projection_matrix.inverse_transform().unwrap();
    //println!("view: {:?}\nproj: {:?}\nviewProj: {:?}\ninvViewProj: {:?}", view_matrix, projection_matrix, view_projection_matrix, inv_view_projection_matrix);

    // :TODO: Think further about how to represent a collection of hetergenous objects uniformly.
    let mut shapes: Vec<Box<Hitable>> = Vec::new();
    shapes.push(Box::new(sphere::Sphere { origin: Point3::new(0., 0., -1.), radius: 0.5, material: Box::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5) }) }));
    shapes.push(Box::new(sphere::Sphere { origin: Point3::new(0., -100.5, -1.), radius: 100., material: Box::new(Lambertian { albedo: vec3(0.8, 0.8, 0.0) }) }));
    shapes.push(Box::new(sphere::Sphere { origin: Point3::new(1., 0., -1.), radius: 0.5, material: Box::new(Metal { albedo: vec3(0.8, 0.6, 0.2), fuzziness: 0.3 }) }));
    shapes.push(Box::new(sphere::Sphere { origin: Point3::new(-1., 0., -1.), radius: 0.5, material: Box::new(Dialectric { refractive_index: 1.5 }) }));
    //shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., -1.), radius: -0.45, material: Box::new(Dialectric { refractive_index: 1.5 }) }));
        
    let mut image: Vec<u8> = Vec::new();

    for y in 0..image_height {
        for x in 0..image_width {
            let mut colour = Vector3::zero();
            for _s in 0..num_samples {
                let sx = (x as f32) + random::<f32>();
                let sy = (y as f32) + random::<f32>();

                 let ndc = Point3::new(
                    (sx as f32 / (image_width as f32 / 2.)) - 1.,
                    (-(sy as f32) / (image_height as f32 / 2.)) + 1.,
                    0.
                );
                let ray_pos = inv_view_projection_matrix.transform_point(ndc);
                let ray_dir = (ray_pos - camera.eye).normalize();

                // :TODO: Defocus blur - tidy up, move some logic into camera struct
                let lens_radius = camera.aperture / 2.;
                let rd = lens_radius * random::random_in_unit_disk();
                let cam_up = view_matrix.transform_vector(vec3(0., 1., 0.));
                let cam_right = ray_dir.cross(cam_up);
                let ray_offset = (cam_up * rd.x) + (cam_right * rd.y);
                let focus_point = ray_pos + (ray_dir * camera.focus_distance);
                let ray_pos = ray_pos + ray_offset;
                let ray_dir = (focus_point - ray_pos).normalize();

                let ray = Ray {
                    origin: ray_pos,
                    direction: ray_dir
                };

                colour += trace(&shapes, &ray, 0);
            }
            colour = colour / (num_samples as f32);

            // Gamma correct & convert to 8bpp
            let colour = vec3(colour.x.sqrt(), colour.y.sqrt(), colour.z.sqrt()) * 255.;
            let Vector3 { x: r, y: g, z: b} = colour;

            image.push(r as u8);
            image.push(g as u8);
            image.push(b as u8);
        }
    }

    write_png_rgb8("output.png", image.as_slice(), (image_width, image_height)).expect("Unable to save PNG");
}

