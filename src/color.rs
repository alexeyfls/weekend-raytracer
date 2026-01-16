use image;
use rand::Rng;
use std::ops::{Add, Div, Mul};
use ultraviolet as uv;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub uv::Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(uv::Vec3::new(r, g, b))
    }

    pub fn black() -> Self {
        Self(uv::Vec3::zero())
    }

    pub fn gamma_correct(self, gamma: f32) -> Self {
        Color(uv::Vec3::new(
            self.0.x.powf(1.0 / gamma),
            self.0.y.powf(1.0 / gamma),
            self.0.z.powf(1.0 / gamma),
        ))
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();

        Color::new(
            rng.random_range(0.0..255.9),
            rng.random_range(0.0..255.9),
            rng.random_range(0.0..255.9),
        )
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

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color(self.0 * rhs.0)
    }
}
