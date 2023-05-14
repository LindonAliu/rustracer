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
    let normal: Vector3D = Vector3D {
        x: intersection_point.x,
        y: intersection_point.y,
        z: 0.,
        w: 1.
    };

    Intersection::new(intersection_point, normal, cylinder.material, ray)
}

#[typetag::serde]
impl Shape for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let pt_cylinder: Vector3D = Vector3D {
            x: sq(ray.direction.x) + sq(ray.direction.y),
            y: 2. * (ray.origin.x * ray.direction.x + ray.origin.y * ray.direction.y),
            z: sq(ray.origin.x) + sq(ray.origin.y) - sq(self.radius),
            w: 1.,
        };

        intersect_polynomial(pt_cylinder).map(|x| cylinder_calcul_intersect(self, ray, x))
    }
}
