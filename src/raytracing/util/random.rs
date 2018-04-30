use cgmath::*;
use rand::{random};
use std::f32;

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

pub fn random_unit_vector() -> Vector3<f32> {
    let z = (2.0 * random::<f32>()) - 1.0;
    let tmp = (1.0 - (z * z)).sqrt();
    let azimuth = random::<f32>() * 2.0 * f32::consts::PI;
    let x = azimuth.cos() * tmp;
    let y = azimuth.sin() * tmp;
    vec3(x, y, z)

    // :TODO: Not sure if this is correct
    // let p = (2. * random::<Vector3<f32>>()) - vec3(1., 1., 1.);
    // let tmp = 1.0 / p.mul_element_wise(p).sum().sqrt();
    // p * tmp
}