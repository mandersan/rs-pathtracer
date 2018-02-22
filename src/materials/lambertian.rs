pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.location + hit.normal + random_in_unit_sphere();
        let scattered_ray = Ray { origin: hit.location, direction: target - hit.location };
        let attenuation = self.albedo;
        Some(Scatter { ray: scattered_ray, attenuation })
    }
}