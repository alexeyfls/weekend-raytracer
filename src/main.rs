use image;
use rand::{distr::Uniform, prelude::*};
use ray::Ray;
use rayon::prelude::*;
use std::{env, f32::consts::PI, fs, path::Path};
use ultraviolet::{self as uv, Lerp};

use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    sphere::Sphere,
};

mod camera;
mod color;
mod hitable;
mod ray;
mod sphere;

const DIMENSION: (u32, u32) = (1920, 1080);

const SAMPLES: usize = 64;

const MAX_BOUNCES: usize = 5;

pub trait RandomInit {
    fn rand(rng: &mut ThreadRng) -> Self;
}

impl RandomInit for uv::Vec3 {
    fn rand(rng: &mut ThreadRng) -> Self {
        let theta = rng.random_range(0.0..2.0 * PI);
        let phi = rng.random_range(-1.0..1.0);
        let ophisq = ((1.0 - phi * phi) as f32).sqrt();
        uv::Vec3::new(ophisq * theta.cos(), ophisq * theta.sin(), phi)
    }
}

fn compute_color(ray: &Ray, world: &HitableList, rng: &mut ThreadRng, bounces: usize) -> Color {
    if bounces >= MAX_BOUNCES {
        return Color::zero();
    }

    if let Some(record) = world.hit(ray, 0.0..100.0) {
        let bounce: ultraviolet::Vec3 = record.n + uv::Vec3::rand(rng);

        compute_color(&Ray::new(record.p, bounce), world, rng, bounces + 1) * 0.5
    } else {
        let dir = ray.direction.clone().normalized();
        let t = 0.5 * (dir.y + 1.0);

        Color(uv::Vec3::lerp(
            &uv::Vec3::one(),
            uv::Vec3::new(0.5, 0.7, 1.0),
            t,
        ))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let save_path = Path::new("./renders").join(&args[1]).with_extension("png");

    if let Some(parent) = save_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create render directory");
    }

    let camera = Camera::new(DIMENSION.0 as f32 / DIMENSION.1 as f32);

    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    world.push(Box::new(Sphere::new(uv::Vec3::new(0.0, 0.0, -1.0), 0.5)));

    let mut pixels = vec![Color::zero(); DIMENSION.0 as usize * DIMENSION.1 as usize];

    pixels.par_iter_mut().enumerate().for_each(|(i, p)| {
        let x = i % DIMENSION.0 as usize;
        let y = (i - x) / DIMENSION.0 as usize;
        let color = (0..SAMPLES)
            .into_iter()
            .map(|_| {
                let mut rng: ThreadRng = rand::rng();
                let uniform = Uniform::new(0.0, 1.0).unwrap();
                let (r1, r2) = (uniform.sample(&mut rng), uniform.sample(&mut rng));
                let uv = uv::Vec3::new(
                    (x as f32 + r1) / DIMENSION.0 as f32,
                    (y as f32 + r2) / DIMENSION.1 as f32,
                    0.0,
                );

                compute_color(&camera.get_ray(uv), &world, &mut rng, 0)
            })
            .fold(Color::zero(), |a, b| a + b);
        *p = color / SAMPLES as f32;
    });

    let mut image = image::RgbImage::new(DIMENSION.0, DIMENSION.1);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let idx = x + (DIMENSION.1 - 1 - y) * DIMENSION.0;
        *pixel = pixels[idx as usize].into();
    }

    image.save(&save_path).expect("Failed to save image");
}
