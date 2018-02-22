use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Ray, Scatterable};

pub struct Sphere {
    pub origin: Point3<f32>,
    pub radius: f32,
    pub material: Box<Scatterable>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
        let sphere_to_ray_origin = ray.origin - self.origin;
        let a = dot(ray.direction, ray.direction);
        let b = dot(sphere_to_ray_origin, ray.direction);
        let c = dot(sphere_to_ray_origin, sphere_to_ray_origin) - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        //println!("{:?} {:?} {:?} {:?} {} {} {} {}", ray.origin, ray.direction, origin, sphere_to_ray_origin, a, b, c, discriminant);

        if discriminant > 0. {
            let tmp = (-b - (b * b - a * c).sqrt()) / a;
            if tmp < interval.max && tmp > interval.min {
                let hit_location = ray.origin + (tmp * ray.direction);
                return Some(Hit {
                    distance: tmp,
                    location: hit_location,
                    normal: (hit_location - self.origin) / self.radius,
                    material: &*self.material,
                });
            }
            let tmp = (-b + (b * b - a * c).sqrt()) / a;
            if tmp < interval.max && tmp > interval.min {
                let hit_location = ray.origin + (tmp * ray.direction);
                return Some(Hit {
                    distance: tmp,
                    location: hit_location,
                    normal: (hit_location - self.origin) / self.radius,
                    material: &*self.material,
                });
            }
        }
        None
    }
}