use cgmath::*;

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>
}

pub struct Interval {
    pub min: f32,
    pub max: f32
}

pub struct Hit<'a> {
    pub distance: f32,
    pub location: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a (Scatterable + 'a), // :TODO: Better undestand lifetime use here
}

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatteredRay>;
}

pub type HitableCollection = Vec<Box<Hitable + Sync>>;