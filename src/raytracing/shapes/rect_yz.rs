use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Ray, ScatteringAndEmitting};

pub struct RectYZ {
	pub y0: f32,
	pub y1: f32,
	pub z0: f32,
	pub z1: f32,
	pub k: f32,
    pub material: Box<ScatteringAndEmitting+Sync>,
}

impl Hitable for RectYZ {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
		let ray_x_to_k = self.k - ray.origin.x;
		let t = ray_x_to_k / ray.direction.x;
		if (t < interval.min) || (t > interval.max) {
			return None;
		}
		let y = ray.origin.y + t * ray.direction.y;
		let z = ray.origin.z + t * ray.direction.z;
		if (y < self.y0) || (y > self.y1) || (z < self.z0) || (z > self.z1) {
			return None;
		}
		Some(Hit {
			distance: t,
			location: Point3::new(self.k, y, z),
			normal: vec3(-ray_x_to_k.signum(), 0., 0.),
			material: &*self.material,
			// uv: vec2((y - self.y0) / (self.y1 - self.y0), (z - self.z0) / (self.z1 - self.z0)),
		})
    }
}