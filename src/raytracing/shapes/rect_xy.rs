use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Ray, ScatteringAndEmitting};

pub struct RectXY {
	pub x0: f32,
	pub x1: f32,
	pub y0: f32,
	pub y1: f32,
	pub k: f32,
    pub material: Box<ScatteringAndEmitting+Sync>,
}

impl Hitable for RectXY {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
		let t = (self.k - ray.origin.z) / ray.direction.z;
		if (t < interval.min) || (t > interval.max) {
			return None;
		}
		let x = ray.origin.x + t * ray.direction.x;
		let y = ray.origin.y + t * ray.direction.y;
		if (x < self.x0) || (x > self.x1) || (y < self.y0) || (y > self.y1) {
			return None;
		}
		Some(Hit {
			distance: t,
			location: Point3::new(x, y, self.k),
			normal: vec3(0., 0., 1.),
			material: &*self.material,
		})
    }
}