//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// sphere
//

use crate::intersection::{Intersection, Ray};
use crate::material::Material;
use crate::shape::Shape;
use crate::vector3d::{Point3D, Vector3D};
use std::cmp::min;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub material: Material
}

fn sq(val: f64) -> f64 {
    val * val
}

fn sphere_calcul_intersect(sphere: &Sphere, ray: &Ray, x: f64) -> Option<Intersection> {
    let intersection_point: Vector3D = ray.origin + ray.direction * x;
    let normal: Vector3D = intersection_point - sphere.center;
    Some (
        Intersection {
            intersection_point: intersection_point,
            distance: (ray.origin - intersection_point).length(),
            normal: if ray.direction.dot(normal) > 0. {
                -normal
            } else {
                normal
            },
            material: sphere.material
        }
    )
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let pt: Vector3D = ray.origin - self.center;
        let pt_share: Vector3D = Vector3D {
            x: sq(ray.direction.x) + sq(ray.direction.y) + sq(ray.direction.z),
            y: (2. * pt.x * ray.direction.x) + (2. * pt.y * ray.direction.y) + (2. * pt.z * ray.direction.y),
            z: sq(pt.x) + sq(pt.y) + sq(pt.z) - sq(self.radius),
        };
        let delta: f64 = sq(pt_share.y) - (4.0 * pt_share.x * pt_share.z);
        let x1: f64 = (-pt_share.y + sq(delta)) / (2. * pt_share.x);
        let x2: f64 = (-pt_share.y - sq(delta)) / (2. * pt_share.x);

        if delta < 0. || (x1 < 0. && x2 < 0. || delta.is_nan()) {
            return None;
        }

        if delta == 0. {
            sphere_calcul_intersect(self, ray, x1)
        } else if x1 >= 0. && x2 < 0. {
            sphere_calcul_intersect(self, ray, x1)
        } else if x2 >= 0. && x1 < 0. {
            sphere_calcul_intersect(self, ray, x2)
        } else {
            sphere_calcul_intersect(self, ray, x1.min(x2))
        }
    }
}