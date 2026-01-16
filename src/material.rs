use crate::{
    Color, RandomInit, Ray,
    hitable::HitRecord,
    math::{f_schlick, f_schlick_c, f0_from_ior, saturate},
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

#[derive(Clone, Copy, Debug)]
pub struct Refractive {
    f0: Color,
    roughness: f32,
    refraction_index: f32,
}

impl Refractive {
    pub fn new(f0: Color, roughness: f32, refraction_index: f32) -> Self {
        Self {
            f0,
            roughness,
            refraction_index,
        }
    }
}

impl Scatterable for Refractive {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, uv::Vec3)> {
        let fuzz = uv::Vec3::rand(rng) * self.roughness;
        let cos_theta = hit.normal.dot(ray.direction.normalized());

        let (refract_normal, eta, cos_theta) = if ray.direction.dot(hit.normal) > 0.0 {
            (hit.normal * -1.0, self.refraction_index, cos_theta)
        } else {
            (hit.normal, 1.0 / self.refraction_index, -cos_theta)
        };

        let f0 = f0_from_ior(self.refraction_index);
        let fresnel = f_schlick(saturate(cos_theta), f0);

        let bounce = if fresnel > rng.random::<f32>() {
            ray.direction.reflected(hit.normal) + fuzz
        } else {
            ray.direction.refracted(refract_normal, eta) + fuzz
        };

        Some((self.f0, bounce))
    }
}

#[derive(Clone, Copy)]
pub enum Material {
    Metal(Metal),
    Diffuse(Diffuse),
    Refractive(Refractive),
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

impl From<Refractive> for Material {
    fn from(material: Refractive) -> Self {
        Self::Refractive(material)
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
            Material::Refractive(m) => m.scatter(ray, hit, rng),
        }
    }
}
