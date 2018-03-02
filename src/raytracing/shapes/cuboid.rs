use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Material, Ray};
use std::f32;

pub struct Cuboid {
    pub transform: Matrix4<f32>,
    pub dimensions: Vector3<f32>,
    corner_min: Point3<f32>,
    corner_max: Point3<f32>,
    pub material: Material,
}

impl Cuboid {
    pub fn new(transform: Matrix4<f32>, dimensions: Vector3<f32>, material: Material) -> Cuboid {
        let half_dimensions = dimensions / 2.;
        let corner_min = Point3::from_vec(-half_dimensions);
        let corner_max =Point3::from_vec( half_dimensions);
        Cuboid { transform, dimensions, corner_min, corner_max, material }
    }
}

impl Hitable for Cuboid {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
        let inverse_transform = self.transform.invert().unwrap();
        let transformed_ray_origin = inverse_transform.transform_point(ray.origin);
        let transformed_ray_direction = inverse_transform.transform_vector(ray.direction);

        let vec1 = (self.corner_min - transformed_ray_origin).div_element_wise(transformed_ray_direction);
        let vec2 = (self.corner_max - transformed_ray_origin).div_element_wise(transformed_ray_direction);
        let tmin = vec1.z.min(vec2.z).max(vec1.y.min(vec2.y).max(vec1.x.min(vec2.x)));
        let tmax = vec1.z.max(vec2.z).min(vec1.y.max(vec2.y).min(vec1.x.max(vec2.x)));
        let mut r = f32::INFINITY;
        if (tmax >= tmin) && (tmax >= 0.) {
            let dist = if tmin >= 0. { tmin } else { tmax };
            if (dist > interval.min) && (dist < interval.max) {
                r = dist;
            }
        }

        if r < f32::INFINITY {
            let location = transformed_ray_origin + (transformed_ray_direction * r);
            //let t = (location - self.origin).div_element_wise(self.dimensions);
            let t = location.to_vec().div_element_wise(self.dimensions);
            let normal = vec3(
                if (t.x.abs() > t.y.abs()) && (t.z.abs() > t.z.abs()) { t.x.signum() } else { 0. },
                if (t.y.abs() > t.x.abs()) && (t.z.abs() > t.z.abs()) { t.y.signum() } else { 0. },
                if (t.z.abs() > t.x.abs()) && (t.z.abs() > t.y.abs()) { t.z.signum() } else { 0. },
            );
        
            return Some(Hit {
                distance: r,
                location: self.transform.transform_point(location),
                normal: self.transform.transform_vector(normal),
                material: &*self.material,
            });
        }
        None
    }
}