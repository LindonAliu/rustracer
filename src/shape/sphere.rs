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
use serde::{Deserialize, Serialize};
use crate::shape::polynomial::{sq, intersect_polynomial};

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub material: Material
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

#[typetag::serde]
impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let pt: Vector3D = ray.origin - self.center;
        let pt_sphere: Vector3D = Vector3D {
            x: sq(ray.direction.x) + sq(ray.direction.y) + sq(ray.direction.z),
            y: (2. * pt.x * ray.direction.x) + (2. * pt.y * ray.direction.y) + (2. * pt.z * ray.direction.z),
            z: sq(pt.x) + sq(pt.y) + sq(pt.z) - sq(self.radius),
            w: 1.,
        };

        if let Some(x) = intersect_polynomial(pt_sphere) {
            sphere_calcul_intersect(self, ray, x)
        } else {
            None
        }
    }
}
