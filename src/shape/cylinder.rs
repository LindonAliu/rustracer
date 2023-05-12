//
// EPITECH PROJECT, 2023
// cylinder
// File description:
// FreeKOSOVO
//

use crate::intersection::{Intersection, Ray};
use crate::material::Material;
use crate::vector3d::{Vector3D};
use crate::shape::Shape;
use serde::{Deserialize, Serialize};
use crate::shape::polynomial::{sq, intersect_polynomial};

#[derive(Serialize, Deserialize)]
pub struct Cylinder {
    pub radius: f64,
    pub material: Material
}

fn cylinder_calcul_intersect(cylinder: &Cylinder, ray: &Ray, x: f64) -> Intersection {
    let intersection_point: Vector3D = ray.origin + ray.direction * x;
    let center: Vector3D = Vector3D {
        x: 0.,
        y: 0.,
        z: intersection_point.z,
        w: 1.
    };
    let normal: Vector3D = intersection_point - center;
    Intersection {
        intersection_point: intersection_point,
        distance: (ray.origin - intersection_point).length(),
        normal: if ray.direction.dot(normal) > 0. {
            -normal
        } else {
            normal
        },
        material: cylinder.material
    }
}

#[typetag::serde]
impl Shape for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let pt_cylinder: Vector3D = Vector3D {
            x: sq(ray.direction.x) + sq(ray.direction.y),
            y: 2. * (ray.origin.x + ray.direction.x + ray.origin.y * ray.direction.y),
            z: sq(ray.origin.x) + sq(ray.origin.y) - sq(self.radius),
            w: 1.,
        };

        if let Some(x) = intersect_polynomial(pt_cylinder) {
            Some(cylinder_calcul_intersect(self, ray, x))
        } else {
            None
        }
    }
}