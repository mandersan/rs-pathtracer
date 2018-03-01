use cgmath::*;
use raytracing::{Hit, Hitable, Interval, Material, Ray};
use std::f32;

pub struct Cuboid {
    pub origin: Point3<f32>,
    pub dimensions: Vector3<f32>,
    corner_min: Point3<f32>,
    corner_max: Point3<f32>,
    pub material: Material,
}

impl Cuboid {
    pub fn new(origin: Point3<f32>, dimensions: Vector3<f32>, material: Material) -> Cuboid {
        let half_dimensions = dimensions / 2.;
        let corner_min = origin - half_dimensions;
        let corner_max = origin + half_dimensions;
        Cuboid { origin, dimensions, corner_min, corner_max, material }
    }
}

impl Hitable for Cuboid {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
        let vec1 = (self.corner_min - ray.origin).div_element_wise(ray.direction);
        let vec2 = (self.corner_max - ray.origin).div_element_wise(ray.direction);
        let tmin = vec1.z.min(vec2.z).max(vec1.y.min(vec2.y).max(vec1.x.min(vec2.x)));
        let tmax = vec1.z.max(vec2.z).min(vec1.y.max(vec2.y).min(vec1.x.max(vec2.x)));
        let mut r = f32::INFINITY;
        if (tmax >= tmin) && (tmax >= 0.) {
            let dist = if tmin >= 0. { tmin } else { tmax };
            if (dist > interval.min) && (dist < interval.max) {
                r = dist;
            }
        }

        if r < f32::INFINITY {
            let location = ray.origin + (ray.direction * r);
            let t = (location - self.origin).div_element_wise(self.dimensions);
            let normal = vec3(
                if (t.x.abs() > t.y.abs()) && (t.z.abs() > t.z.abs()) { t.x.signum() } else { 0. },
                if (t.y.abs() > t.x.abs()) && (t.z.abs() > t.z.abs()) { t.y.signum() } else { 0. },
                if (t.z.abs() > t.x.abs()) && (t.z.abs() > t.y.abs()) { t.z.signum() } else { 0. },
            );
        
            return Some(Hit {
                distance: r,
                location,
                normal,
                material: &*self.material,
            });
        }
        None
    }
}
/*
Maths::Vector4 normalAtIntersection;

 Maths::Vector4 adjustedIntersectionPoint = rIntersectionPoint / m_dimensions;
 float x = adjustedIntersectionPoint.GetX();
 float y = adjustedIntersectionPoint.GetY();
 float z = adjustedIntersectionPoint.GetZ();
 float magX = fabsf( x );
 float magY = fabsf( y );
 float magZ = fabsf( z );
 if (    ( magX > magY )
         &&      ( magX > magZ ) )
 {
         normalAtIntersection.SetX( ( x > 0.0f ) ? 1.0f : -1.0f );
 }
 else if ( magY > magZ )
 {
         normalAtIntersection.SetY( ( y > 0.0f ) ? 1.0f : -1.0f );
 }
 else
 {
         normalAtIntersection.SetZ( ( z > 0.0f ) ? 1.0f : -1.0f );
 }*/