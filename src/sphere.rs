use crate::{
    hitable::{HitRecord, Hitable},
    material::Material,
    ray::Ray,
};
use std::ops::Range;
use ultraviolet::{self as uv};

pub struct Sphere {
    center: uv::Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: uv::Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let h = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant >= 0.0 {
            let sqrt = discriminant.sqrt();

            let mut t = (-h - sqrt) / a;

            if t_range.contains(&t) {
                let p = ray.point_at(t);
                let n = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, n, self.material));
            }

            t = (-h + sqrt) / a;

            if t_range.contains(&t) {
                let p = ray.point_at(t);
                let n = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, n, self.material));
            }
        }

        None
    }
}
