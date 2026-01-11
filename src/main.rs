use ultraviolet::{self as uv, Lerp};

use ray::Ray;

mod ray;

fn hit_sphere(center: uv::Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction.clone());
    let b = 2.0 * oc.clone().dot(ray.direction.clone());
    let c = oc.clone().dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn color(ray: Ray) -> uv::Vec3 {
    if hit_sphere(uv::Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return uv::Vec3::new(1.0, 0.0, 0.0);
    }

    let normalized_dir = ray.direction.clone().normalized();
    let t = 0.5 * (normalized_dir.y + 1.0);

    uv::Vec3::one().lerp(uv::Vec3::new(0.5, 0.7, 1.0), t)
}

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let lower_left_corner = uv::Vec3::new(-1.0, -1.0, -1.0);
    let horizontal = uv::Vec3::new(2.0, 0.0, 0.0);
    let vertical = uv::Vec3::new(0.0, 2.0, 0.0);
    let origin = uv::Vec3::new(0.0, 0.0, 0.0);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / IMAGE_WIDTH as f32;
            let v = j as f32 / IMAGE_HEIGHT as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let c = color(ray);

            let ir = (255.99 * c.x) as u8;
            let ig = (255.99 * c.y) as u8;
            let ib = (255.99 * c.z) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
