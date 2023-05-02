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

#[derive(Debug)]
pub struct Plane {
    pub pos: Point3D,
    pub normal: Vector3D,
    pub material: Material
}

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
            Intersection {
                intersection_point: intersection_point,
                normal: if divi > 0. {
                    self.normal * -1.
                } else {
                    self.normal
                },
                distance: ray.origin.dot(intersection_point),
                material: self.material
            }
        )
    }
}

// struct plane *p = obj;
// double d = -vector_product(&p->pos, &p->normal);
// double divi = vector_product(&r->direction, &p->normal);
// double t = -(vector_product(&p->normal, &r->origin) + d) / divi;

// if (t <= 0 || divi == 0)
//     return false;
// out->intersection = (struct vector) {r->origin.x + t * r->direction.x,
//     r->origin.y + t * r->direction.y, r->origin.z + t * r->direction.z};
// out->distance = vector_distance(&r->origin, &out->intersection);
// if (divi > 0)
//     out->normal = (struct vector) {-p->normal.x,
//         -p->normal.y, -p->normal.z};
// else
//     out->normal = p->normal;
// return true;