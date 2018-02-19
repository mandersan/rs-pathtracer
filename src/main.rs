/*
:TODO:
- Multiple shapes.
- Shading model.
- Lights.
- Efficient scene organisation.
- Etc.
*/

extern crate cgmath;
extern crate image;
extern crate rand;

use cgmath::*;
use image::ColorType;
use image::png::PNGEncoder;
use rand::{random, Rand};
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

fn intersect_test_sphere(sphere: &Sphere, ray: &Ray, interval: &Interval) -> Option<Hit>
{
    let sphere_to_ray_origin = ray.origin - sphere.origin;
    let a = dot(ray.direction, ray.direction);
    let b = dot(sphere_to_ray_origin, ray.direction);
    let c = dot(sphere_to_ray_origin, sphere_to_ray_origin) - (sphere.radius * sphere.radius);
    let discriminant = b * b - a * c;

    //println!("{:?} {:?} {:?} {:?} {} {} {} {}", ray.origin, ray.direction, sphere.origin, sphere_to_ray_origin, a, b, c, discriminant);

    if discriminant > 0. {
        let tmp = (-b - (b * b - a * c).sqrt()) / a;
        if tmp < interval.max && tmp > interval.min {
            let hit_location = ray.origin + (tmp * ray.direction);
            return Some(Hit {
                distance: tmp,
                location: hit_location,
                normal: (hit_location - sphere.origin) / sphere.radius
            });
        }
        let tmp = (-b + (b * b - a * c).sqrt()) / a;
        if tmp < interval.max && tmp > interval.min {
            let hit_location = ray.origin + (tmp * ray.direction);
            return Some(Hit {
                distance: tmp,
                location: hit_location,
                normal: (hit_location - sphere.origin) / sphere.radius
            });
        }
    }
    None    
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
        eye: Point3::new(0., 0., 2.),
        target: Point3::new(0., 0., 1.),
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

    let test_sphere = Sphere { origin: Point3::new(0., 0., 1.), radius: 0.3 };
    
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

                let hit = intersect_test_sphere(&test_sphere, &ray, &Interval { min: 0.0, max: f32::MAX });
                colour += match hit {
                    None => {
                        let t = 0.5 * (ray_dir.y + 1.0);
                        ((1.0 - t) * vec3(1., 1., 1.)) + (t * vec3(0.5, 0.7, 1.0))
                    },
                    Some(hit) => {
                        // :NOTE: Just showing normals
                        0.5 * vec3(hit.normal.x + 1., hit.normal.y + 1., hit.normal.z + 1.)
                    }
                };
            }
            colour = colour / (num_samples as f32);

            //let Vector3 { x: r, y: g, z: b} = colour;

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

