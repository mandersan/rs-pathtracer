use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Ray, ScatteringAndEmitting};

pub struct RectXZ {
	pub x0: f32,
	pub x1: f32,
	pub z0: f32,
	pub z1: f32,
	pub k: f32,
    pub material: Box<ScatteringAndEmitting+Sync>,
}

impl Hitable for RectXZ {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
		let t = (self.k - ray.origin.y) / ray.direction.y;
		if (t < interval.min) || (t > interval.max) {
			return None;
		}
		let x = ray.origin.x + t * ray.direction.x;
		let z = ray.origin.z + t * ray.direction.z;
		if (x < self.x0) || (x > self.x1) || (z < self.z0) || (z > self.z1) {
			return None;
		}
		Some(Hit {
			distance: t,
			location: Point3::new(x, self.k, z),
			normal: vec3(0., 1., 0.),
			material: &*self.material,
			// uv: vec2((x - self.x0) / (self.x1 - self.x0), (z - self.z0) / (self.z1 - self.z0)),
		})
    }
}