extern crate lodepng;
extern crate cgmath;

use std::fs::File;
use std::io::Write;
use cgmath::*;

fn main() {
    // :TODO:
    // - Preview output in window.
    // - Command line args parsing (output to file/window).
    // - 

    let image_width = 320;
    let image_height = 256;
    let image_aspect = image_width as f32 / image_height as f32;

    let camera_eye = vec3(0, 0, 0);
    let camera_target = vec3(0, 0, -1);
    let camera_up = vec3(0, 1, 0);
    let camera_fov = Deg(60.0);
    let camera_near = 0.01;
    let camera_far = 100.;

    let view = lookAt(camera_eye, camera_target, camera_up);
    let projection = perspective(camera_fov, image_aspect, camera_near, camera_far);
    
    // :TODO: Viewport matrix (using 'lookAt' function)
    //      float4x4 viewMatrix = lookat(CamPos, TgtPos, float3{ 0.f, 1.f, 0.f });
    // :TODO: For each pixel, build ray (NDC? So, -1 to 1 in each axis) and transform using viewProj matrix.
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

