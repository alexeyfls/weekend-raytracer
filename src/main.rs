use image;
use rand::{distr::Uniform, prelude::*};
use rayon::prelude::*;
use std::{
    env,
    f32::consts::PI,
    fs, io,
    path::{Path, PathBuf},
};
use ultraviolet::{self as uv, Lerp};

use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    material::{Diffuse, Metal, Refractive, Scatterable},
    ray::Ray,
    sphere::Sphere,
};

mod camera;
mod color;
mod hitable;
mod material;
mod math;
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
        return Color::black();
    }

    if let Some(record) = world.hit(ray, 0.001..100.0) {
        let scatter = record.material.scatter(ray, &record, rng);
        if let Some((attenuation, bounce)) = scatter {
            return compute_color(&Ray::new(record.point, bounce), world, rng, bounces + 1)
                * attenuation;
        }

        return Color::black();
    }

    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y + 1.0);

    Color(uv::Vec3::lerp(
        &uv::Vec3::one(),
        uv::Vec3::new(0.5, 0.7, 1.0),
        t,
    ))
}

fn resolve_save_to_path() -> io::Result<PathBuf> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing filename argument"))?;

    let save_path = Path::new("./renders").join(filename).with_extension("png");

    if let Some(parent) = save_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(save_path)
}

fn setup() -> (Camera, HitableList) {
    let camera = Camera::new(DIMENSION.0 as f32 / DIMENSION.1 as f32);
    let mut world = HitableList::new();

    let pink_diffuse = Diffuse::new(Color::new(0.7, 0.3, 0.4), 0.0);
    let ground = Diffuse::new(Color::new(0.35, 0.3, 0.45), 0.2);
    let gold = Metal::new(Color::new(1.0, 0.9, 0.5), 0.0);
    let gold_rough = Metal::new(Color::new(1.0, 0.9, 0.5), 0.2);
    let silver = Metal::new(Color::new(0.9, 0.9, 0.9), 0.05);
    let glass = Refractive::new(Color::new(0.9, 0.9, 0.9), 0.0, 1.5);
    let glass_rough = Refractive::new(Color::new(0.9, 0.9, 0.9), 0.2, 1.5);

    world.push(Box::new(Sphere::new(
        uv::Vec3::new(0.0, -200.5, -1.0),
        200.0,
        ground.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(0.0, 0.0, -1.0),
        0.5,
        silver.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        pink_diffuse.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(1.0, -0.25, -1.0),
        0.25,
        gold.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(0.4, -0.375, -0.5),
        0.125,
        glass.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(0.2, -0.4, -0.35),
        0.1,
        glass.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(-0.25, -0.375, -0.15),
        0.125,
        glass_rough.into(),
    )));
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(-0.5, -0.375, -0.5),
        0.125,
        gold_rough.into(),
    )));

    (camera, world)
}

fn main() -> Result<(), io::Error> {
    let save_path = resolve_save_to_path()?;

    let (camera, world) = setup();
    let mut pixels = vec![Color::black(); DIMENSION.0 as usize * DIMENSION.1 as usize];

    pixels.par_iter_mut().enumerate().for_each(|(i, p)| {
        let mut rng: ThreadRng = rand::rng();
        let uniform = Uniform::new(0.0, 1.0).unwrap();

        let x = i % DIMENSION.0 as usize;
        let y = (i - x) / DIMENSION.0 as usize;
        let color = (0..SAMPLES)
            .into_iter()
            .map(|_| {
                let (r1, r2) = (uniform.sample(&mut rng), uniform.sample(&mut rng));
                let uv = uv::Vec3::new(
                    (x as f32 + r1) / DIMENSION.0 as f32,
                    (y as f32 + r2) / DIMENSION.1 as f32,
                    0.0,
                );

                compute_color(&camera.get_ray(uv), &world, &mut rng, 0)
            })
            .fold(Color::black(), |a, b| a + b);
        *p = color / SAMPLES as f32;
    });

    let mut image = image::RgbImage::new(DIMENSION.0, DIMENSION.1);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let idx = x + (DIMENSION.1 - 1 - y) * DIMENSION.0;
        *pixel = pixels[idx as usize].gamma_correct(2.0).into();
    }

    image.save(&save_path).expect("Failed to save image");

    Ok(())
}
