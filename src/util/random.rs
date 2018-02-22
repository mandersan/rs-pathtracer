use cgmath::*;
use rand::{random};

pub fn random_in_unit_sphere() -> Vector3<f32> {
    // :TODD: Check that the initial random vector has values in the range 0..1
    loop {
        let p = (2.0 * random::<Vector3<f32>>()) - vec3(1., 1., 1.);
        if p.magnitude2() < 1. {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let p = 2.0 * vec3(random::<f32>(), random::<f32>(), 0.) - vec3(1., 1., 0.);
        if dot(p, p) < 1. {
            return p;
        }
    }
}