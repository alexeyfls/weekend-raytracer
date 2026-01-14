use crate::{
    hitable::{HitRecord, Hitable},
    material::Material,
};
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
    fn hit(&self, ray: &crate::ray::Ray, t_range: std::ops::Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let b = oc.dot(ray.direction.clone());
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant >= 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;

            if t_range.contains(&t) {
                let p = ray.point_at(t);
                let n = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, n, self.material));
            }

            t = (-b + discriminant.sqrt()) / a;

            if t_range.contains(&t) {
                let p = ray.point_at(t);
                let n = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, n, self.material));
            }
        }

        None
    }
}
