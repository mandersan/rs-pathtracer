use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Ray, Scatterable};

pub struct Plane {
    pub origin: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: Box<Scatterable>,
}

impl Hitable for Plane {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
        if dot(ray.direction, self.normal) < 0. {
            let numerator = dot(self.normal, self.origin - ray.origin);
            let denominator = dot(self.normal, ray.direction);
            let r = numerator / denominator;

            if (r > interval.min) && (r < interval.max) && (denominator != 0.) {
                return Some(Hit {
                    distance: r,
                    location: ray.origin + (ray.direction * r),
                    normal: self.normal,
                    material: &*self.material,
                });
            }
        }
        None
    }
}