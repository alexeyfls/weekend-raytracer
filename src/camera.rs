use crate::Ray;
use ultraviolet as uv;

#[derive(Clone, Copy)]
pub struct Camera {
    lower_left: uv::Vec3,
    full_size: uv::Vec3,
    origin: uv::Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Camera {
            lower_left: uv::Vec3::new(-aspect_ratio * 0.5, -0.5, -1.0),
            full_size: uv::Vec3::new(aspect_ratio, 1.0, 0.0),
            origin: uv::Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn get_ray(&self, uv: uv::Vec3) -> Ray {
        Ray::new(self.origin.clone(), self.lower_left + self.full_size * uv)
    }
}
