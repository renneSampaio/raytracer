#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::left() * 4.0;
    let vertical = Vec3::up() * 2.0;
    let origin = Vec3::zero();

    let mut data = Vec::<u8>::with_capacity((4 * WIDTH * HEIGHT) as usize);

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(&ray);

            data.push((255.0 * col.r()) as u8);
            data.push((255.0 * col.g()) as u8);
            data.push((255.0 * col.b()) as u8);
            data.push(255);
        }
    }

    write_image("test.png", WIDTH, HEIGHT, &data);
}

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.direction;

    // Puts t in the range 0..1
    let t = 0.5 * (dir.y() + 1.0);

    // Gradient from blue to white
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
