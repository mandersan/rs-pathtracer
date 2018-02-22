use cgmath::*;

pub struct Metal {
    albedo: Vector3<f32>,
    fuzziness: f32,
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let scattered_ray = Ray{ origin: hit.location, direction: reflected + self.fuzziness * random_in_unit_sphere() };
        let attenuation = self.albedo;
        if dot(scattered_ray.direction, hit.normal) > 0.0 {
            Some(Scatter { ray: scattered_ray, attenuation })
        } else {
            None
        }        
    }
}