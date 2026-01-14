use crate::{
    Color, RandomInit, Ray,
    hitable::HitRecord,
    math::{f_schlick, f_schlick_c, saturate},
};
use rand::{self, Rng, rngs::ThreadRng};
use ultraviolet as uv;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut ThreadRng)
    -> Option<(Color, uv::Vec3)>;
}

#[derive(Clone, Copy, Debug)]
pub struct Diffuse {
    albedo: Color,
    roughness: f32,
}

impl Diffuse {
    pub fn new(albedo: Color, roughness: f32) -> Self {
        Self { albedo, roughness }
    }
}

impl Scatterable for Diffuse {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, uv::Vec3)> {
        let norm = hit.normal.clone();
        let cos = saturate(
            hit.normal
                .normalized()
                .dot(ray.direction.clone().normalized() * -1.0),
        );
        let fresnel = f_schlick(cos, 0.04);
        let bounce = if rng.random::<f32>() > fresnel {
            norm + uv::Vec3::rand(rng)
        } else {
            ray.direction.reflected(norm.clone()) + (uv::Vec3::rand(rng) * self.roughness)
        };

        Some((self.albedo, bounce))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    f0: Color,
    roughness: f32,
}

impl Metal {
    pub fn new(f0: Color, roughness: f32) -> Self {
        Self { f0, roughness }
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, uv::Vec3)> {
        let bounce =
            ray.direction.reflected(hit.normal.clone()) + uv::Vec3::rand(rng) * self.roughness;
        let cos = saturate(
            hit.normal
                .normalized()
                .dot(ray.direction.clone().normalized() * -1.0),
        );
        let attenuation = f_schlick_c(cos, self.f0);

        Some((attenuation, bounce))
    }
}

#[derive(Clone, Copy)]
pub enum Material {
    Metal(Metal),
    Diffuse(Diffuse),
}

impl From<Diffuse> for Material {
    fn from(material: Diffuse) -> Self {
        Self::Diffuse(material)
    }
}

impl From<Metal> for Material {
    fn from(material: Metal) -> Self {
        Self::Metal(material)
    }
}

impl Scatterable for Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, uv::Vec3)> {
        match self {
            Material::Diffuse(m) => m.scatter(ray, hit, rng),
            Material::Metal(m) => m.scatter(ray, hit, rng),
        }
    }
}
