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

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fov: Deg<f32>,
    near: f32,
    far: f32,
}

fn write_png_rgb8(filename: &str, pixels: &[u8], dimensions: (u32, u32))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, dimensions.0, dimensions.1, ColorType::RGB(8))?;
    Ok(())
}

struct Sphere {
    origin: Point3<f32>,
    radius: f32
}

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>
}

struct Interval {
    min: f32,
    max: f32
}

struct Hit {
    distance: f32,
    location: Point3<f32>,
    normal: Vector3<f32>
}

trait Hitable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>
    {
        let sphere_to_ray_origin = ray.origin - self.origin;
        let a = dot(ray.direction, ray.direction);
        let b = dot(sphere_to_ray_origin, ray.direction);
        let c = dot(sphere_to_ray_origin, sphere_to_ray_origin) - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        //println!("{:?} {:?} {:?} {:?} {} {} {} {}", ray.origin, ray.direction, origin, sphere_to_ray_origin, a, b, c, discriminant);

        if discriminant > 0. {
            let tmp = (-b - (b * b - a * c).sqrt()) / a;
            if tmp < interval.max && tmp > interval.min {
                let hit_location = ray.origin + (tmp * ray.direction);
                return Some(Hit {
                    distance: tmp,
                    location: hit_location,
                    normal: (hit_location - self.origin) / self.radius
                });
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < interval.max && tmp > interval.min {
                let hit_location = ray.origin + (tmp * ray.direction);
                return Some(Hit {
                    distance: tmp,
                    location: hit_location,
                    normal: (hit_location - self.origin) / self.radius
                });
            }
        }
        None
    }
}

fn random_in_unit_sphere() -> Vector3<f32>
{
    // :TODD: Check that the initial random vector has values in the range 0..1
    loop {
        let p = (2.0 * random::<Vector3<f32>>()) - vec3(1., 1., 1.);
        if p.magnitude2() < 1. {
            return p;
        }
    }
}

fn hit(shapes: &Vec<Box<Hitable>>, ray: &Ray, interval: &Interval) -> Option<Hit>
{
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

fn trace(shapes: &Vec<Box<Hitable>>, ray: &Ray) -> Vector3<f32> {
    let hit = hit(shapes, ray, &Interval { min: 0.001, max: f32::MAX });
    let colour = match hit {
        None => {
            let t = 0.5 * (ray.direction.y + 1.0);
            ((1.0 - t) * vec3(1., 1., 1.)) + (t * vec3(0.5, 0.7, 1.0))
        },
        Some(hit) => {
            let target = hit.location + hit.normal + random_in_unit_sphere();
            let new_ray = Ray { origin: hit.location, direction: target - hit.location };
            0.5 * trace(shapes, &new_ray)
            // :NOTE: Normals
            //0.5 * vec3(hit.normal.x + 1., hit.normal.y + 1., hit.normal.z + 1.)
        }
    };
    colour
}

fn main() {
    // :TODO:
    // - Preview output in window.
    // - Command line args parsing (output to file/window).
    // - 

    let image_width = 320;
    let image_height = 200;
    let image_aspect = image_width as f32 / image_height as f32;
    let num_samples = 64;

    let camera = Camera {
        eye: Point3::new(0., 0., 0.75),
        target: Point3::new(0., 0., -1.),
        up: vec3(0., 1., 0.),
        fov: Deg(60.0),
        near: 0.01,
        far: 100.,
    };

    let view_matrix = Matrix4::look_at(camera.eye, camera.target, camera.up);
    let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
    let view_projection_matrix = projection_matrix * view_matrix;
    let inv_view_projection_matrix = view_projection_matrix.inverse_transform().unwrap();
    //println!("view: {:?}\nproj: {:?}\nviewProj: {:?}\ninvViewProj: {:?}", view_matrix, projection_matrix, view_projection_matrix, inv_view_projection_matrix);

    // :TODO: Think further about how to represent a collection of hetergenous objects uniformly.
    let mut shapes: Vec<Box<Hitable>> = Vec::new();
    shapes.push(Box::new(Sphere { origin: Point3::new(0., 0., -1.), radius: 0.5 }));
    shapes.push(Box::new(Sphere { origin: Point3::new(0., -100.5, -1.), radius: 100. }));
    
    let mut image: Vec<u8> = Vec::new();

    for y in 0..image_height {
        for x in 0..image_width {
            let mut colour = Vector3::zero();
            for s in 0..num_samples {
                let sx = (x as f32) + random::<f32>();
                let sy = (y as f32) + random::<f32>();

                 let ndc = Point3::new(
                    (sx as f32 / (image_width as f32 / 2.)) - 1.,
                    (-(sy as f32) / (image_height as f32 / 2.)) + 1.,
                    0.
                );
                let ray_pos = inv_view_projection_matrix.transform_point(ndc);
                let ray_dir = (ray_pos - camera.eye).normalize();

                let ray = Ray {
                    origin: ray_pos,
                    direction: ray_dir
                };

                colour += trace(&shapes, &ray);
            }
            colour = colour / (num_samples as f32);

            //let Vector3 { x: r, y: g, z: b} = colour;

            // Gamma correct
            let colour = vec3(colour.x.sqrt(), colour.y.sqrt(), colour.z.sqrt());
      
            let r = colour.x * 255.;
            let g = colour.y * 255.;
            let b = colour.z * 255.;

            image.push(r as u8);
            image.push(g as u8);
            image.push(b as u8);
        }
    }

    write_png_rgb8("output.png", image.as_slice(), (image_width, image_height)).expect("Unable to save PNG");
}

