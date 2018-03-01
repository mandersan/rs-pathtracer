// /*
// :TODO:
// - Multiple shapes - better/more efficient storage/organisation.
// - Shading model.
// - Lights.
// - Efficient scene organisation.
// - Multi-threaded rendering.
// - Spectral path tracing (single scalar per ray? Randomly pick a wavelength per bounce?)
// - Preview output in window.
// - Command line args parsing (output to file/window).
// */

extern crate cgmath;
extern crate crossbeam;
extern crate image;
extern crate num_cpus;
extern crate rand;
extern crate sdl2;
mod raytracing;

use cgmath::*;
// use image::ColorType;
// use image::png::PNGEncoder;
use rand::{random};
use raytracing::materials::{Dialectric, Lambertian, Metal};
use raytracing::{HitableCollection, Ray};
use raytracing::shapes::{Plane, Sphere};
use raytracing::util::{random};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use std::fs::File;

// fn write_png_rgb8(filename: &str, pixels: &[u8], dimensions: (u32, u32))
//     -> Result<(), std::io::Error>
// {
//     let output = File::create(filename)?;
//     let encoder = PNGEncoder::new(output);
//     encoder.encode(pixels, dimensions.0, dimensions.1, ColorType::RGB(8))?;
//     Ok(())
// }

fn render(
    pixels: &mut [u8],
    top_left: (usize, usize),
    bounds: (usize, usize),
    num_samples: u32,
    image_width: usize,
    image_height: usize,
    view_matrix: &Matrix4<f32>,
    inv_view_projection_matrix: &Matrix4<f32>,
    camera: &raytracing::cameras::Camera,
    shapes: & HitableCollection,
)
{
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let mut colour = Vector3::zero();
            for _s in 0..num_samples {
                let sx = ((x + top_left.0) as f32) + random::<f32>();
                let sy = ((y + top_left.1) as f32) + random::<f32>();

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

                colour += raytracing::tracing::trace(shapes, &ray, 0);
            }
            colour = colour / (num_samples as f32);

            // Gamma correct & convert to 8bpp
            let colour = vec3(colour.x.sqrt(), colour.y.sqrt(), colour.z.sqrt()) * 255.;
            let Vector3 { x: r, y: g, z: b} = colour;

            let base = ((y * image_width) + x) * 3;
            pixels[base + 0] = r as u8;
            pixels[base + 1] = g as u8;
            pixels[base + 2] = b as u8;
        }
    }
}

fn main() {
    // Set up image output & camera
    let window_width = 640;
    let window_height = 480;
    let image_width = 320;
    let image_height = 240;
    let image_aspect = image_width as f32 / image_height as f32;
    let num_samples = 4;

    // Build scene
    // :TODO: Think further about how to represent a collection of hetergenous objects uniformly.
    let mut shapes: HitableCollection = Vec::new();
    shapes.push(Box::new(Sphere { origin: Point3::new(0., 0., 0.), radius: 0.5, material: Box::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5) }) }));
    shapes.push(Box::new(Plane { origin: Point3::new(0., -0.5, 0.), normal: vec3(0., 1., 0.), material: Box::new(Lambertian { albedo: vec3(0.2, 0.5, 0.2) }) }));
    shapes.push(Box::new(Sphere { origin: Point3::new(1., 0., 0.), radius: 0.5, material: Box::new(Metal { albedo: vec3(0.8, 0.6, 0.2), fuzziness: 0.3 }) }));
    shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., 0.), radius: 0.5, material: Box::new(Dialectric { refractive_index: 1.5 }) }));
    //shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., 0.), radius: -0.45, material: Box::new(Dialectric { refractive_index: 1.5 }) }));

    let mut cam_pos = Point3::new(0., 0.2, 1.75);



    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", window_width, window_height)
        .position_centered()
        //.opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        let rotation = Matrix3::from_angle_y(Deg(1.));
        cam_pos = rotation.transform_point(cam_pos);

        let camera = raytracing::cameras::util::create_camera(
            cam_pos,
            Point3::new(0., 0., 0.),
            vec3(0., 1., 0.),
            Deg(60.),
            0.01,
            100.,
            0.2
        );
        let view_matrix = Matrix4::look_at(camera.eye, camera.target, camera.up);
        let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
        let view_projection_matrix = projection_matrix * view_matrix;
        let inv_view_projection_matrix = view_projection_matrix.inverse_transform().unwrap();

        let mut image: Vec<u8> = vec![0; image_width * image_height * 3];


        let thread_count = num_cpus::get();
        
        let rows_per_band = image_height / thread_count;

        {
            let bands: Vec<&mut [u8]> = image.chunks_mut(rows_per_band * image_width * 3).collect();
            crossbeam::scope(|scope| {
                let camera_ref = &camera;
                let shapes_ref = &shapes;
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;
                    let height = band.len() / (image_width * 3);
                    let top_left = (0, top);
                    let band_bounds = (image_width, height);
                    scope.spawn(move || {
                        render(band, top_left, band_bounds, num_samples, image_width, image_height, &view_matrix, &inv_view_projection_matrix, camera_ref, shapes_ref);
                    });
                }
            });
        }


        let mut texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, image_width as u32, image_height as u32).unwrap();
        // Create a red-green gradient
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..image_height as usize {
                for x in 0..image_width as usize {
                    let offset = y*pitch + x*3;
                    let offset_in = y*(image_width * 3) as usize + x*3;
                    buffer[offset + 0] = image[offset_in + 0];
                    buffer[offset + 1] = image[offset_in + 1];
                    buffer[offset + 2] = image[offset_in + 2];
                }
            }
        }).unwrap();

        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, window_width, window_height))).unwrap();
        canvas.present();
    }
}