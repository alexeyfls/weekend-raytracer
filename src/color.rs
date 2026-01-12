use image;
use std::ops::{Add, Div, Mul};
use ultraviolet as uv;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub uv::Vec3);

impl Color {
    pub fn zero() -> Self {
        Self(uv::Vec3::zero())
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        image::Rgb([
            (color.0.x * 255.0) as u8,
            (color.0.y * 255.0) as u8,
            (color.0.z * 255.0) as u8,
        ])
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0)
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color(self.0 / rhs)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color(self.0 * rhs)
    }
}
