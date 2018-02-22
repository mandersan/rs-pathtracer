use cgmath::*;

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov: Deg<f32>,
    pub near: f32,
    pub far: f32,
    pub aperture: f32,
    pub focal_distance: f32,
}