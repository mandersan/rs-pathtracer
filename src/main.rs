extern crate lodepng;

use std::fs::File;
use std::io::Write;

fn main() {
    // :TODO:
    // 1. PPM output of a basic image.
    // 2. 

    let width = 320;
    let height = 256;
    
    let mut image: Vec<u8> = Vec::new();

    let ppm_max_value = 255;
    let ppm_header = format!("P3\n{} {}\n{}\n", width, height, ppm_max_value);
    let mut output_file = File::create("output.ppm")
        .expect("Unable to open output file");
    output_file.write_all(ppm_header.as_bytes())
        .expect("Error writing header");

    for y in 0..height {
        for x in 0..width {
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

    lodepng::encode_file("output.png", image.as_slice(), width, height, lodepng::ColorType::RGB, 8)
        .expect("Unable to save PNG");
}

