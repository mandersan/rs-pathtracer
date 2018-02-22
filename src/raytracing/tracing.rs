use cgmath::*;
use raytracing::{Hitable, Hit, Interval, Ray};
use std::f32;

pub fn hit<'a>(shapes: &'a Vec<Box<Hitable>>, ray: &Ray, interval: &Interval) -> Option<Hit<'a>> {
    let mut hit_result: Option<Hit> = None;
    let mut closest = interval.max;
    for shape in shapes
    {
        if let Some(hit) = shape.hit(ray, &Interval { min: interval.min, max: closest }) {
            closest = hit.distance;
            hit_result = Some(hit);
        }
    }
    return hit_result;
}

pub fn trace(shapes: &Vec<Box<Hitable>>, ray: &Ray, depth: u32) -> Vector3<f32> {
    let hit = hit(shapes, ray, &Interval { min: 0.001, max: f32::MAX });
    let colour = match hit {
        None => {
            let t = 0.5 * (ray.direction.y + 1.0);
            ((1.0 - t) * vec3(1., 1., 1.)) + (t * vec3(0.5, 0.7, 1.0))
        },
        Some(hit) => {
            if depth < 50 {
                let scatter_result = hit.material.scatter(ray, &hit);
                let colour = match scatter_result {
                    None => vec3(0., 0., 0.),
                    Some(scatter_result) => scatter_result.attenuation.mul_element_wise(trace(shapes, &scatter_result.ray, depth + 1))
                };
                colour
            } else {
                vec3(0., 0., 0.)
            }
        }
    };
    colour
}