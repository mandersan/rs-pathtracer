use cgmath::*;
use raytracing::{Emitting, Hit, Ray, Scattering, ScatteringAndEmitting, ScatteredRay};

pub struct DiffuseLight {
	pub colour: Vector3<f32>,
}

impl Emitting for DiffuseLight {
	fn emit(&self, _u: f32, _v: f32, _p: &Point3<f32>) -> Vector3<f32> {
		// :TODO:
		self.colour
	}
}

impl Scattering for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<ScatteredRay> {
        None
    }
}

impl ScatteringAndEmitting for DiffuseLight {}