#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

mod geometry;
mod ray;
mod vec3;

use crate::geometry::{HitInfo, Hitable, Sphere};
use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::left() * 4.0;
    let vertical = Vec3::up() * 2.0;
    let origin = Vec3::zero();

    let mut world = Vec::<Box<Hitable>>::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.75, 0.0, -1.0), 0.15)));
    world.push(Box::new(Sphere::new(Vec3::new(-0.85, 0.0, -1.0), 0.25)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut data = Vec::<u8>::with_capacity((4 * WIDTH * HEIGHT) as usize);

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(&ray, &world);

            data.push((255.0 * col.r()) as u8);
            data.push((255.0 * col.g()) as u8);
            data.push((255.0 * col.b()) as u8);
            data.push(255);
        }
    }

    write_image("test.png", WIDTH, HEIGHT, &data);
}

fn color(ray: &Ray, world: &[Box<Hitable>]) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, std::f32::MAX) {
        return (hit.normal + Vec3::new(1.0, 1.0, 1.0)) / 2.0;
    }

    let dir = ray.direction;

    // Puts t in the range 0..1
    let t = 0.5 * (dir.y() + 1.0);

    // Gradient from blue to white
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.lenght_squared();
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.lenght_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (-b - discriminant.sqrt()) / (2.0 * a)
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
