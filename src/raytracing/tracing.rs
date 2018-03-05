use cgmath::*;
use raytracing::{BoxedHitable, Hit, Interval, Ray};
use std::f32;

pub fn hit<'a>(shapes: &'a[BoxedHitable], ray: &Ray, interval: &Interval) -> Option<Hit<'a>> {
    let mut hit_result: Option<Hit> = None;
    let mut closest = interval.max;
    for shape in shapes
    {
        if let Some(hit) = shape.hit(ray, &Interval { min: interval.min, max: closest }) {
            closest = hit.distance;
            hit_result = Some(hit);
        }
    }
    hit_result
}

pub fn trace(shapes: &[BoxedHitable], ray: &Ray, depth: u32) -> Vector3<f32> {
    let hit = hit(shapes, ray, &Interval { min: 0.001, max: f32::MAX });
    match hit {
        None => {
            // let t = 0.5 * (ray.direction.y + 1.0);
            // ((1.0 - t) * vec3(1., 1., 1.)) + (t * vec3(0.5, 0.7, 1.0))
            vec3(0., 0., 0.)
        },
        Some(hit) => {
            // :TODO: UVs
            let emitted = hit.material.emit(0., 0., &hit.location);
            if depth < 50 {
                let scatter_result = hit.material.scatter(ray, &hit);
                match scatter_result {
                    None => emitted,//vec3(0., 0., 0.),
                    Some(scatter_result) => emitted + scatter_result.attenuation.mul_element_wise(trace(shapes, &scatter_result.ray, depth + 1))
                }
            } else {
                emitted//vec3(0., 0., 0.)
            }
        }
    }
}