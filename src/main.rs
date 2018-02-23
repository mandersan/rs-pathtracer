/*
:TODO:
- Multiple shapes - better/more efficient storage/organisation.
- Shading model.
- Lights.
- Efficient scene organisation.
- Multi-threaded rendering.
- Spectral path tracing (single scalar per ray? Randomly pick a wavelength per bounce?)
- Preview output in window.
- Command line args parsing (output to file/window).
*/

extern crate cgmath;
extern crate image;
extern crate rand;
mod raytracing;

use cgmath::*;
use image::ColorType;
use image::png::PNGEncoder;
use rand::{random};
use raytracing::materials::{Dialectric, Lambertian, Metal};
use raytracing::{Hitable, Ray};
use raytracing::shapes::{Plane, Sphere};
use raytracing::util::{random};
use std::fs::File;

fn write_png_rgb8(filename: &str, pixels: &[u8], dimensions: (u32, u32))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, dimensions.0, dimensions.1, ColorType::RGB(8))?;
    Ok(())
}

fn main() {
    // Set up image output & camera
    let image_width = 640;
    let image_height = 400;
    let image_aspect = image_width as f32 / image_height as f32;
    let num_samples = 50;
    let camera = raytracing::cameras::util::create_camera(
        Point3::new(0., 0., 0.75),
        Point3::new(0., 0., -1.),
        vec3(0., 1., 0.),
        Deg(60.),
        0.01,
        100.,
        0.4
    );
    let view_matrix = Matrix4::look_at(camera.eye, camera.target, camera.up);
    let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
    let view_projection_matrix = projection_matrix * view_matrix;
    let inv_view_projection_matrix = view_projection_matrix.inverse_transform().unwrap();

    // Build scene
    // :TODO: Think further about how to represent a collection of hetergenous objects uniformly.
    let mut shapes: Vec<Box<Hitable>> = Vec::new();
    shapes.push(Box::new(Sphere { origin: Point3::new(0., 0., -1.), radius: 0.5, material: Box::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5) }) }));
    shapes.push(Box::new(Plane { origin: Point3::new(0., -0.5, 0.), normal: vec3(0., 1., 0.), material: Box::new(Lambertian { albedo: vec3(0.2, 0.2, 0.2) }) }));
    shapes.push(Box::new(Sphere { origin: Point3::new(1., 0., -1.), radius: 0.5, material: Box::new(Metal { albedo: vec3(0.8, 0.6, 0.2), fuzziness: 0.3 }) }));
    shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., -1.), radius: 0.5, material: Box::new(Dialectric { refractive_index: 1.5 }) }));
    //shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., -1.), radius: -0.45, material: Box::new(Dialectric { refractive_index: 1.5 }) }));

    // Raytrace scene
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
                let focus_point = ray_pos + (ray_dir * camera.focal_distance);
                let ray_pos = ray_pos + ray_offset;
                let ray_dir = (focus_point - ray_pos).normalize();

                let ray = Ray {
                    origin: ray_pos,
                    direction: ray_dir
                };

                colour += raytracing::tracing::trace(&shapes, &ray, 0);
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

    // Save image as file
    write_png_rgb8("output.png", image.as_slice(), (image_width, image_height)).expect("Unable to save PNG");
}