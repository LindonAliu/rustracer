//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// plane
//

use crate::intersection::{Intersection, Ray};
use crate::material::Material;
use crate::vector3d::{Point3D, Vector3D};
use crate::shape::Shape;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub pos: Point3D,
    pub normal: Vector3D,
    pub material: Material
}

#[typetag::serde]
impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let d: f64 = -self.pos.dot(self.normal);
        let divi: f64 = ray.direction.dot(self.normal);
        let t: f64 = -(self.normal.dot(ray.origin) + d) / divi;

        if t <= 0. || divi == 0. {
            return None;
        }
        let intersection_point: Vector3D = ray.origin + ray.direction * t;
        Some (
            Intersection::new(intersection_point, self.normal, self.material, ray)
        )
    }
}
