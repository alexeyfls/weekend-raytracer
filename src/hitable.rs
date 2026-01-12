use crate::ray::Ray;
use std::ops::{Deref, Range};
use ultraviolet as uv;

#[allow(dead_code)]
pub struct HitRecord {
    pub t: f32,
    pub p: uv::Vec3,
    pub n: uv::Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: uv::Vec3, n: uv::Vec3) -> Self {
        Self { t, p, n }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

pub struct HitableList(Vec<Box<dyn Hitable + Send + Sync>>);

impl HitableList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, hitable: Box<dyn Hitable + Send + Sync>) {
        self.0.push(hitable)
    }
}

impl Deref for HitableList {
    type Target = Vec<Box<dyn Hitable + Send + Sync>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_range.end;

        for hitable in self.iter() {
            if let Some(hit) = hitable.hit(ray, t_range.start..closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
