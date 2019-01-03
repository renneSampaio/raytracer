#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;
use rand::prelude::*;

mod camera;
mod geometry;
mod ray;
mod vec3;

use crate::camera::Camera;
use crate::geometry::{HitInfo, Hitable, Sphere};
use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 100;
    const NUMBER_OF_STEPS: u32 = 32;

    let camera = Camera::new();

    let mut world = Vec::<Box<Hitable>>::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.75, -0.2, -1.0), 0.15)));
    world.push(Box::new(Sphere::new(Vec3::new(-0.85, -0.2, -1.0), 0.25)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut data = Vec::<u8>::with_capacity((4 * WIDTH * HEIGHT) as usize);

    let mut rng = rand::thread_rng();

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let mut col = Vec3::zero();

            for _ in 0..NUMBER_OF_STEPS {
                let jitter_x: f32 = rng.gen();
                let jitter_y: f32 = rng.gen();
                let u = ((x as f32) + jitter_x) / WIDTH as f32;
                let v = ((y as f32) + jitter_y) / HEIGHT as f32;

                let ray = camera.get_ray(u, v);

                col += color(&ray, &world, &mut rng);
            }

            col /= NUMBER_OF_STEPS as f32;
            data.push((255.0 * col.r().sqrt()) as u8);
            data.push((255.0 * col.g().sqrt()) as u8);
            data.push((255.0 * col.b().sqrt()) as u8);
            data.push(255);
        }
    }

    write_image("test.png", WIDTH, HEIGHT, &data);
}

fn color(ray: &Ray, world: &[Box<Hitable>], rng: &mut rand::RngCore) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, std::f32::MAX) {
        let target = hit.p + hit.normal + Vec3::random_in_unit_sphere(rng);
        return color(&Ray::new(hit.p, target - hit.p), world, rng) / 2.0;
    }

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
