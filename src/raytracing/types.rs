use cgmath::*;

pub trait ScatteringAndEmitting : Scattering + Emitting {}

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
    pub material: &'a (ScatteringAndEmitting + 'a), // :TODO: Better undestand lifetime use here
    // pub uv: Point2<f32>,
}

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}

pub type HitableCollection = Vec<Box<Hitable + Sync>>;

pub trait Scattering {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatteredRay>;
}

pub trait Emitting {
    fn emit(&self, _u: f32, _v: f32, _p: &Point3<f32>) -> Vector3<f32> {
        vec3(0., 0., 0.)
    }
}

pub type Material = Box<ScatteringAndEmitting+Sync>;