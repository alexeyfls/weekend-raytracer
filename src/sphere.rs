use crate::hitable::{HitRecord, Hitable};
use ultraviolet::{self as uv};

pub struct Sphere {
    center: uv::Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: uv::Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_range: std::ops::Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction.clone());
        let b = oc.clone().dot(ray.direction.clone());
        let c = oc.clone().dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant >= 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;

            if t_range.contains(&t) {
                let p = ray.point_at(t);
                let n = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, n));
            }

            t = (-b + discriminant.sqrt()) / a;

            if t_range.contains(&t) {
                let p = ray.point_at(t);
                let n = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, n));
            }
        }

        None
    }
}
