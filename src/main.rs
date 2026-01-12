use image;
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
use std::{env, fs, path::Path};
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

const SAMPLES: usize = 48;

fn compute_color(ray: &Ray, world: &HitableList) -> Color {
    if let Some(record) = world.hit(ray, 0.0..100.0) {
        Color(uv::Vec3::broadcast(0.5) * (uv::Vec3::one() + record.n))
    } else {
        let mut dir = ray.direction.clone();
        dir.normalize();
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
                let mut rng = rand::rng();
                let uv = uv::Vec3::new(
                    (x as f32 + rng.random::<f32>()) / DIMENSION.0 as f32,
                    (y as f32 + rng.random::<f32>()) / DIMENSION.1 as f32,
                    0.0,
                );
                compute_color(&camera.get_ray(uv), &world)
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
