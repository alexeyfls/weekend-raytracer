use crate::Color;
use ultraviolet as uv;

pub fn f0_from_ior(ior: f32) -> f32 {
    let f0 = (1.0 - ior) / (1.0 + ior);
    f0 * f0
}

pub fn f_schlick(cos: f32, f0: f32) -> f32 {
    f0 + (1.0 - f0) * (1.0 - cos).powi(5)
}

pub fn f_schlick_c(cos: f32, f0: Color) -> Color {
    let f0 = f0.0;
    let out_v = f0 + (uv::Vec3::broadcast(1.0) - f0) * (1.0 - cos).powi(5);
    Color(out_v)
}

pub fn saturate(v: f32) -> f32 {
    v.min(1.0).max(0.0)
}
