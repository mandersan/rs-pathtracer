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
use raytracing::cameras::{Camera};
use raytracing::materials::{Dialectric, DiffuseLight, Lambertian, Metal};
use raytracing::{BoxedHitable, HitableCollection, Ray};
use raytracing::shapes::{Cuboid, Plane, RectXY, RectXZ, RectYZ, Sphere};
use raytracing::util::{random};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
// use std::fs::File;

// fn write_png_rgb8(filename: &str, pixels: &[u8], dimensions: (u32, u32))
//     -> Result<(), std::io::Error>
// {
//     let output = File::create(filename)?;
//     let encoder = PNGEncoder::new(output);
//     encoder.encode(pixels, dimensions.0, dimensions.1, ColorType::RGB(8))?;
//     Ok(())
// }

fn scene_test() -> HitableCollection {
    let mut shapes: HitableCollection = Vec::new();
    shapes.push(Box::new(Sphere { origin: Point3::new(0., 0., 0.), radius: 0.5, material: Box::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5) }) }));
    shapes.push(Box::new(Plane { origin: Point3::new(0., -0.5, 0.), normal: vec3(0., 1., 0.), material: Box::new(Lambertian { albedo: vec3(0.2, 0.5, 0.2) }) }));
    shapes.push(Box::new(Sphere { origin: Point3::new(1., 0., 0.), radius: 0.5, material: Box::new(Metal { albedo: vec3(0.8, 0.6, 0.2), fuzziness: 0.3 }) }));
    shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., 0.), radius: 0.5, material: Box::new(Dialectric { refractive_index: 1.5 }) }));
    //shapes.push(Box::new(Sphere { origin: Point3::new(-1., 0., 0.), radius: -0.45, material: Box::new(Dialectric { refractive_index: 1.5 }) }));
    shapes.push(Box::new(RectXZ { x0: -0.5, x1: 0.5, z0: -0.5, z1: 0.5, k: 2., material: Box::new(DiffuseLight { colour: vec3(4., 4., 4.) }) }));
    shapes
}

fn scene_cornell_box() -> HitableCollection {
    let mut shapes: HitableCollection = Vec::new();
    // Walls
    shapes.push(Box::new(RectYZ { y0: 0., y1: 555., z0: 0., z1: 555., k: 555., material: Box::new(Lambertian { albedo: vec3(0.12, 0.45, 0.15) }) }));
    shapes.push(Box::new(RectYZ { y0: 0., y1: 555., z0: 0., z1: 555., k: 0., material: Box::new(Lambertian { albedo: vec3(0.65, 0.05, 0.05) }) }));
    shapes.push(Box::new(RectXZ { x0: 0., x1: 555., z0: 0., z1: 555., k: 0., material: Box::new(Lambertian { albedo: vec3(0.73, 0.73, 0.73) }) }));
    shapes.push(Box::new(RectXZ { x0: 0., x1: 555., z0: 0., z1: 555., k: 555., material: Box::new(Lambertian { albedo: vec3(0.73, 0.73, 0.73) }) }));
    shapes.push(Box::new(RectXY { x0: 0., x1: 555., y0: 0., y1: 555., k: 555., material: Box::new(Lambertian { albedo: vec3(0.73, 0.73, 0.73) }) }));
    // Light
    shapes.push(Box::new(RectXZ { x0: 213., x1: 343., z0: 227., z1: 332., k: 554., material: Box::new(DiffuseLight { colour: vec3(15., 15., 15.) }) }));
    // Boxes
    let transform = Matrix4::from_translation(vec3(212.5, 82.5, 147.5)) * Matrix4::from_angle_y(Deg(-18.));
    shapes.push(Box::new(Cuboid::new(transform, vec3(165., 165., 165.), Box::new(Lambertian { albedo: vec3(0.73, 0.73, 0.73) }))));
    let transform = Matrix4::from_translation(vec3(347.5, 165., 377.5)) * Matrix4::from_angle_y(Deg(15.));
    shapes.push(Box::new(Cuboid::new(transform, vec3(165., 330., 165.), Box::new(Lambertian { albedo: vec3(0.73, 0.73, 0.73) }))));   
    shapes
}
fn camera_cornell_box() -> Camera {
    raytracing::cameras::util::create_camera(
        Point3::new(278., 278., -800.),
        Point3::new(278., 278., 0.),
        vec3(0., 1., 0.),
        Deg(40.),
        0.01,
        100.,
        0.,
    )
}

fn render(
    pixels: &mut [f32],
    top_left: (usize, usize),
    bounds: (usize, usize),
    num_samples: u32,
    image_width: usize,
    image_height: usize,
    view_matrix: &Matrix4<f32>,
    inv_view_projection_matrix: &Matrix4<f32>,
    camera: &raytracing::cameras::Camera,
    shapes: &[BoxedHitable],
    ray_count: &mut u64,
)
{
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let mut colour = Vector3::zero();
            for _s in 0..num_samples {
                let sx = ((x + top_left.0) as f32) + random::<f32>();
                let sy = ((y + top_left.1) as f32) + random::<f32>();

                let ndc = Point3::new(
                    (sx / (image_width as f32 / 2.)) - 1.,
                    (-sy / (image_height as f32 / 2.)) + 1.,
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

                colour += raytracing::tracing::trace(shapes, &ray, 0, ray_count);
            }
            colour /= num_samples as f32;

            let Vector3 { x: r, y: g, z: b} = colour;

            let base = ((y * image_width) + x) * 3;
            pixels[base] = r;
            pixels[base + 1] = g;
            pixels[base + 2] = b;
        }
    }
}

fn main() {
    // Set up image output & camera
    let window_width = 640;
    let window_height = 400;
    let image_width = 640;
    let image_height = 400;
    let image_aspect = image_width as f32 / image_height as f32;
    let num_samples = 1;//20;

    let mut total_samples = 0.0;

    let num_pixels = image_width * image_height;
    let mut accumulated_image: Vec<f32> = vec![0.0; num_pixels * 3];

    // Build scene
    // :TODO: Think further about how to represent a collection of hetergenous objects uniformly.
    //let shapes = scene_test();
    let shapes = scene_cornell_box();

    //let mut cam_pos = Point3::new(0., 0.2, 1.75);



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

        // let rotation = Matrix3::from_angle_y(Deg(1.));
        // cam_pos = rotation.transform_point(cam_pos);
        // let camera = raytracing::cameras::util::create_camera(
        //     cam_pos,
        //     Point3::new(0., 0., 0.),
        //     vec3(0., 1., 0.),
        //     Deg(60.),
        //     0.01,
        //     100.,
        //     0.2
        // );
        let camera = camera_cornell_box();
        let view_matrix = Matrix4::look_at(camera.eye, camera.target, camera.up);
        let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
        let view_projection_matrix = projection_matrix * view_matrix;
        let inv_view_projection_matrix = view_projection_matrix.inverse_transform().unwrap();

        let mut image: Vec<f32> = vec![0.0; num_pixels * 3];

        

        let thread_count = num_cpus::get();
        let rows_per_band = image_height / thread_count;

        let mut ray_counts: Vec<u64> = vec![0; thread_count];
        let start_time = Instant::now();

        {
            let bands: Vec<&mut [f32]> = image.chunks_mut(rows_per_band * image_width * 3).collect();
            crossbeam::scope(|scope| {
                let camera_ref = &camera;
                let shapes_ref = &shapes;
                let ray_counts_ref = &mut ray_counts;
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;
                    let height = band.len() / (image_width * 3);
                    let top_left = (0, top);
                    let band_bounds = (image_width, height);
                    let tmp = ray_counts.get_mut(i).unwrap();
                    scope.spawn(move || {
                        let mut ray_count = 0;
                        render(band, top_left, band_bounds, num_samples, image_width, image_height, &view_matrix, &inv_view_projection_matrix, camera_ref, shapes_ref, &mut ray_count);
                        ray_counts_ref[i] = ray_count;
                    });
                }
            });
        }

        for p in 0..(num_pixels * 3) as usize {
            accumulated_image[p] = (accumulated_image[p] * (total_samples / (total_samples + 1.0))) + (image[p] * (1.0 / (total_samples + 1.0)));
        }
        total_samples += 1.0;


        let mut texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, image_width as u32, image_height as u32).unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..image_height as usize {
                for x in 0..image_width as usize {
                    let offset = y*pitch + x*3;
                    let offset_in = y*(image_width * 3) as usize + x*3;
                    // Gamma correct & convert to 8bpp
                    buffer[offset] = (accumulated_image[offset_in].min(1.0).sqrt() * 255.) as u8;
                    buffer[offset + 1] = (accumulated_image[offset_in + 1].min(1.0).sqrt() * 255.) as u8;
                    buffer[offset + 2] = (accumulated_image[offset_in + 2].min(1.0).sqrt() * 255.) as u8;
                }
            }
        }).unwrap();

        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, window_width, window_height))).unwrap();
        canvas.present();

        let current_time = Instant::now();
        let duration = current_time.duration_since(start_time);
        let elapsed_time = duration.as_secs() as f64 + (duration.subsec_nanos() as f64 * 1e-9);
        let ray_count = ray_counts.iter().fold(0,|a, &b| a + b);
        let rays_per_second = ray_count as f64 / elapsed_time;
        println!("Rays/sec: {} (rays: {} elapsed_time: {})", rays_per_second, ray_count, elapsed_time);
    }
}