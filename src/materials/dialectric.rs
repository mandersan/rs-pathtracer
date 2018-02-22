use cgmath::*;

pub struct Dialectric {
    refractive_index: f32,
}

impl Scatterable for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = reflect(ray.direction, hit.normal);
        let attenuation = vec3(1., 1., 1.);

        let outward_normal;
        let ni_over_nt;
        let cosine;
        if dot(ray.direction, hit.normal) > 0. {
            outward_normal = -hit.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * dot(ray.direction, hit.normal) / ray.direction.magnitude();
        } else {
            outward_normal = hit.normal;
            // :TODO: This assumes air (idx=1) to medium, will need to ensure correct index is used if light is crossing boundary between two mediums
            ni_over_nt = 1. / self.refractive_index;
            cosine = -dot(ray.direction, hit.normal) / ray.direction.magnitude();
        }

        let refracted = refract(ray.direction, outward_normal, ni_over_nt);
        let reflect_probability = match refracted {
            None => 1.0,
            Some(_refracted) => schlick(cosine, self.refractive_index),
        };

        let scattered = if random::<f32>() < reflect_probability {
            Scatter { ray: Ray { origin: hit.location, direction: reflected }, attenuation }
        } else {
            Scatter { ray: Ray { origin: hit.location, direction: refracted.unwrap() }, attenuation }
        };

        Some(scattered)
    }
}