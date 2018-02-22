use cgmath::*;
use raytracing::cameras::Camera;

pub fn create_camera(
	eye: Point3<f32>,
	target: Point3<f32>,
	up: Vector3<f32>,
	fov: Deg<f32>,
	near: f32,
	far: f32,
	aperture: f32,
) -> Camera {
    Camera {
        eye,
        target,
        up,
        fov,
        near,
        far,
        aperture,
        focal_distance: (target - eye).magnitude(),
    }
}