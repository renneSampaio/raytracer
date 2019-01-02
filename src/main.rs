#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

mod vec3;

use crate::vec3::Vec3;

fn main() {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 100;

    let mut data = Vec::<u8>::with_capacity((4 * WIDTH * HEIGHT) as usize);
    let d_height = 255 / HEIGHT;
    let d_width = 255 / WIDTH;

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let v = Vec3::new(
                (d_width * x) as f32,
                (d_height * y) as f32,
                (255 / 5) as f32,
            );

            data.push(v[0] as u8);
            data.push(v[1] as u8);
            data.push(v[2] as u8);
            data.push(255);
        }
    }

    write_image("test.png", WIDTH, HEIGHT, &data);
}

fn write_image(path: &str, width: u32, height: u32, data: &[u8]) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap();
}
