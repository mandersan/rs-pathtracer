extern crate lodepng;
extern crate cgmath;

use std::fs::File;
use std::io::Write;
use cgmath::*;

struct Camera {
    eye: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
    fov: Deg<f32>,
    near: f32,
    far: f32,
}

fn look_at(eye: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let axis_z = (target - eye).normalize();
    let axis_x = axis_z.cross(up).normalize();
    let axis_y = axis_x.cross(axis_z);
    return Matrix4::from_cols(
        vec4(axis_x.x, axis_y.x, -axis_z.x, 0.),
        vec4(axis_x.y, axis_y.y, -axis_z.y, 0.),
        vec4(axis_x.z, axis_y.z, -axis_z.z, 0.),
        vec4(-dot(axis_x, eye), -dot(axis_y, eye), dot(axis_z, eye), 1.)
    );
}


fn main() {
    let tmp = vec3(0.0,0.0,0.0) - vec3(0.0,0.0,0.0);

    // :TODO:
    // - Preview output in window.
    // - Command line args parsing (output to file/window).
    // - 

    let image_width = 320;
    let image_height = 256;
    let image_aspect = image_width as f32 / image_height as f32;

    let camera = Camera {
        eye: vec3(0., 0., 0.),
        target: vec3(0., 0., -1.),
        up: vec3(0., 1., 0.),
        fov: Deg(60.0),
        near: 0.01,
        far: 100.,
    };

    // :TODO: http://www.songho.ca/opengl/gl_transform.html
    //  Possible todo the perspective divide & conversion to window coords in a matrix?
    let screen_matrix = Matrix4::zero();
    let view_matrix = look_at(camera.eye, camera.target, camera.up);
    let projection_matrix = perspective(camera.fov, image_aspect, camera.near, camera.far);
    let screen_view_projection_matrix = screen_matrix * projection_matrix * view_matrix;
    let inv_screen_view_projection_matrix = screen_view_projection_matrix.inverse_transform();

    /*
        In: (screenX, screenY)
        |
        screen -> NDC
        NDC -> view
        view -> projection
        |
        Out: (worldX, worldY, worldZ)
    */



    // :TODO: Viewport matrix (using 'lookAt' function)
    //      float4x4 viewMatrix = lookat(CamPos, TgtPos, float3{ 0.f, 1.f, 0.f });
    // :TODO: For each pixel, build ray (NDC? So, -1 to 1 in each axis) and transform using viewProj matrix.
    //      Basically screen space (0..width, 0..height) to world space.
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

