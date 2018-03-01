use cgmath::*;
use raytracing::{Emitting, Hit, Ray, Scattering, ScatteringAndEmitting, ScatteredRay};
use raytracing::util::{random};

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Scattering for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<ScatteredRay> {
        let target = hit.location + hit.normal + random::random_in_unit_sphere();
        let scattered_ray = Ray { origin: hit.location, direction: target - hit.location };
        let attenuation = self.albedo;
        Some(ScatteredRay { ray: scattered_ray, attenuation })
    }
}

impl Emitting for Lambertian {}
impl ScatteringAndEmitting for Lambertian {}