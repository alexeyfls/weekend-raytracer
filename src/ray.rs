#![allow(dead_code)]

use ultraviolet as uv;

pub struct Ray {
    pub origin: uv::Vec3,
    pub direction: uv::Vec3,
}

impl Ray {
    pub fn new(origin: uv::Vec3, direction: uv::Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> uv::Vec3 {
        self.direction.mul_add(uv::Vec3::new(t, t, t), self.origin)
    }
}
