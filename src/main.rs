use image;
use ray::Ray;
use std::{env, fs, path::Path};
use ultraviolet::{self as uv, Lerp};

use crate::{
    color::Color,
    hitable::{Hitable, HitableList},
    sphere::Sphere,
};

mod color;
mod hitable;
mod ray;
mod sphere;

const DIMENSION: (f32, f32) = (960.0, 540.0);

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

    let mut image = image::RgbImage::new(DIMENSION.0 as u32, DIMENSION.1 as u32);

    let top_left = uv::Vec3::new(-DIMENSION.0 / DIMENSION.1, 1.0, -1.0);
    let view_full = uv::Vec3::new(DIMENSION.0 / DIMENSION.1 * 2.0, -2.0, 0.0);
    let origin = uv::Vec3::new(0.0, 0.0, 0.0);

    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        uv::Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    world.push(Box::new(Sphere::new(uv::Vec3::new(0.0, 0.0, -1.0), 0.5)));

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let uv = uv::Vec3::new(
            x as f32 / DIMENSION.0 as f32,
            y as f32 / DIMENSION.1 as f32,
            0.0,
        );
        let ray = Ray::new(origin.clone(), top_left + (view_full * uv));
        let col = compute_color(&ray, &world);
        *pixel = col.into();
    }

    let save_path = Path::new("./renders").join(&args[1]).with_extension("png");

    if let Some(parent) = save_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create render directory");
    }

    println!("{:#?}", &save_path);

    image.save(&save_path).expect("Failed to save image");
}
