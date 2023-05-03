//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// triangle
//

use crate::intersection::{Intersection, Ray};
use crate::material::Material;
use crate::shape::Shape;
use crate::vector3d::{Vector3D};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D,
    pub normal: Vector3D,
    pub d: f64,
    pub normal_sq: f64,
    pub material: Material
}

fn calcul_normal(intersection: Vector3D, a:Vector3D, b:Vector3D) -> Vector3D {
    let normal: Vector3D = Vector3D {
        x: (a.y - intersection.y) * (b.z - intersection.z) - (a.z - intersection.z) * (b.y - intersection.y),
        y: (a.z - intersection.z) * (b.x - intersection.x) - (a.x - intersection.x) * (b.z - intersection.z),
        z: (a.x - intersection.x) * (b.y - intersection.y) - (a.x - intersection.y) * (b.x - intersection.x),
    };
    normal
}

fn is_in_triangle(triangle: &Triangle, intersect: Vector3D, normal: Vector3D) -> bool {
    let tmp: Vector3D = calcul_normal(intersect, triangle.b, triangle.c);
    let alpha = normal.dot(tmp) / triangle.normal_sq;
    let tmp2: Vector3D = calcul_normal(intersect, triangle.c, triangle.a);
    let beta = normal.dot(tmp2) / triangle.normal_sq;
    let gamma = 1. - alpha - beta;

    0. <= beta && beta <= 1. && 0. <= alpha && alpha <= 1. && 0. <= gamma && gamma <= 1.
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let divi = ray.direction.dot(self.normal);
        let s = -(self.normal.dot(ray.origin) + self.d) / divi;
        let intersection_point: Vector3D = ray.origin + ray.direction * s;
        if s <= 0. || divi == 0. {
            return None;
        }
        if !is_in_triangle(self, intersection_point, self.normal) {
            return None;
        }
        Some (
            Intersection {
                intersection_point: intersection_point,
                distance: (ray.origin - intersection_point).length(),
                normal: if self.normal.dot(ray.direction) > 0. {
                    -self.normal
                } else {
                    self.normal
                },
                material: self.material
            }
        )
    }
}
