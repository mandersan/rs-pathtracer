use cgmath::*;
use raytracing::{Hit, Ray, Scatterable, ScatteredRay};
use raytracing::util::{maths, random};

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzziness: f32,
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatteredRay> {
        let reflected = maths::reflect(ray.direction.normalize(), hit.normal);
        let scattered_ray = Ray{ origin: hit.location, direction: reflected + self.fuzziness * random::random_in_unit_sphere() };
        let attenuation = self.albedo;
        if dot(scattered_ray.direction, hit.normal) > 0.0 {
            Some(ScatteredRay { ray: scattered_ray, attenuation })
        } else {
            None
        }        
    }
}