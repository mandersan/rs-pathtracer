extern crate lodepng;
extern crate cgmath;

use std::fs::File;
use std::io::Write;
use cgmath::*;

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    fov: Deg<f32>,
    near: f32,
    far: f32,
}

fn main() {
    // :TODO:
    // - Preview output in window.
    // - Command line args parsing (output to file/window).
    // - 

    let image_width = 4;//320;
    let image_height = 4;//256;
    let image_aspect = image_width as f32 / image_height as f32;

    let camera = Camera {
        eye: Point3::new(0., 0., 0.),
        target: Point3::new(0., 0., -1.),
        up: vec3(0., 1., 0.),
        fov: Deg(60.0),
        near: 0.01,
        far: 100.,
    };

    // :TODO: http://www.songho.ca/opengl/gl_transform.html
    let view_matrix = Matrix4::look_at(camera.eye, camera.target, camera.up);
    let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
    let view_projection_matrix = projection_matrix * view_matrix;
    let inv_view_projection_matrix = view_projection_matrix.inverse_transform().unwrap();

    /*
        In: (screenX, screenY)
        |
        screen -> NDC
        NDC -> view
        view -> projection
        |
        Out: (worldX, worldY, worldZ)
    */

    // :TODO: Get simple sky & sphere intersection working.

    
    let mut image: Vec<u8> = Vec::new();

    let ppm_max_value = 255;
    let ppm_header = format!("P3\n{} {}\n{}\n", image_width, image_height, ppm_max_value);
    let mut output_file = File::create("output.ppm")
        .expect("Unable to open output file");
    output_file.write_all(ppm_header.as_bytes())
        .expect("Error writing header");

    for y in 0..image_height {
        for x in 0..image_width {
            let ndc = Point3::new(
                (x as f32 / ((image_width - 1) as f32 / 2.)) - 1.,
                (y as f32 / ((image_height - 1) as f32 / 2.)) - 1.,
                0.
            );

            let ray_pos = inv_view_projection_matrix.transform_point(ndc);
            //let ray_pos = inv_view_projection_matrix * ndc;
            let ray_dir = inv_view_projection_matrix.transform_vector(vec3(0., 0., -1.));

            println!("({}, {}): {:?} {:?} {:?}", x, y, ndc, ray_pos, ray_dir);


            let r = (x % 16) * 16;
            let g = 0;
            let b = (y % 16) * 16;
            let element = format!("{} {} {}\n", r, g, b);
            output_file.write_all(element.as_bytes())
                .expect("Unable to write element");

            image.push(r as u8);
            image.push(g as u8);
            image.push(b as u8);
        }
    }

    lodepng::encode_file("output.png", image.as_slice(), image_width, image_height, lodepng::ColorType::RGB, 8)
        .expect("Unable to save PNG");
}

