#![allow(dead_code)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;
use rand::prelude::*;

mod camera;
mod geometry;
mod material;
mod ray;
mod vec3;

use crate::camera::Camera;
use crate::geometry::{HitInfo, Hitable, Sphere};
use crate::material::Material::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const NUMBER_OF_STEPS: u32 = 100;

    let look_from = Vec3::new(11.0, 2.0, 2.5);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let apertune = 0.05;
    let dist_to_focus = (look_from - look_at).lenght();

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::up(),
        25.0,
        WIDTH as f32 / HEIGHT as f32,
        apertune,
        dist_to_focus,
    );

    let mut rng = rand::thread_rng();

    let world = random_scene(&mut rng);

    let mut data = Vec::<u8>::with_capacity((4 * WIDTH * HEIGHT) as usize);

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let mut col = Vec3::zero();

            for _ in 0..NUMBER_OF_STEPS {
                let jitter_x: f32 = rng.gen();
                let jitter_y: f32 = rng.gen();
                let u = ((x as f32) + jitter_x) / WIDTH as f32;
                let v = ((y as f32) + jitter_y) / HEIGHT as f32;

                let ray = camera.get_ray(u, v, &mut rng);

                col += color(&ray, &world, &mut rng, 0);
            }

            col /= NUMBER_OF_STEPS as f32;
            data.push((255.0 * col.r().sqrt()) as u8);
            data.push((255.0 * col.g().sqrt()) as u8);
            data.push((255.0 * col.b().sqrt()) as u8);
            data.push(255);

            print!("\r\r");
            print!(
                "{:.2}% Completed                                   ",
                (data.len() * 25) as f32 / (WIDTH * HEIGHT) as f32
            );
        }
    }

    write_image("test.png", WIDTH, HEIGHT, &data);
}

fn color(ray: &Ray, world: &[Box<Hitable>], rng: &mut rand::RngCore, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, std::f32::MAX) {
        let scatter_data = hit.material.scatter(ray, &hit, rng);
        if depth < 50 && scatter_data.is_some() {
            let (scatter, attenuation) = scatter_data.unwrap();
            return attenuation * color(&scatter, world, rng, depth + 1);
        } else {
            return Vec3::zero();
        }
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

fn random_scene(rng: &mut rand::RngCore) -> Vec<Box<Hitable>> {
    let mut world = Vec::<Box<Hitable>>::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mat_choice: f32 = rng.gen();

            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).lenght() > 0.9 {
                if mat_choice < 0.8 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Lambertian {
                            albedo: Vec3::new(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            ),
                        },
                    )));
                } else if mat_choice < 0.95 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            fuzz: 0.5 * rng.gen::<f32>(),
                        },
                    )));
                } else {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Dieletric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dieletric { ref_idx: 1.5 },
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.4),
        },
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    world
}
