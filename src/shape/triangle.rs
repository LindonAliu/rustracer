//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// triangle
//

use crate::intersection::{Intersection, Ray};
use crate::material::Material;
use crate::shape::Shape;
use crate::vector3d::{Vector3D, Point3D};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
struct SerTriangle {
    a: Point3D,
    b: Point3D,
    c: Point3D,
    material: Material
}

#[derive(Serialize, Deserialize)]
#[serde(from = "SerTriangle")]
pub struct Triangle {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,

    pub normal: Vector3D,
    pub d: f64,
    pub normal_sq: f64,

    pub material: Material
}

impl From<SerTriangle> for Triangle {
    fn from(triangle: SerTriangle) -> Self {
        Triangle::new(triangle.a, triangle.b, triangle.c, triangle.material)
    }
}

impl Triangle {
    pub fn new(a: Point3D, b: Point3D, c: Point3D, material: Material) -> Self {
        let normal = calcul_normal(a, b, c);
        let d = -a.dot(normal);
        let normal_sq = normal.dot(normal);
        Triangle {
            a,
            b,
            c,
            normal,
            d,
            normal_sq,
            material
        }
    }
}

fn calcul_normal(intersection: Vector3D, a:Vector3D, b:Vector3D) -> Vector3D {
    let diff_a = a - intersection;
    let diff_b = b - intersection;

    let normal: Vector3D = Vector3D {
        x: (diff_a.y) * (diff_b.z) - (diff_a.z) * (diff_b.y),
        y: (diff_a.z) * (diff_b.x) - (diff_a.x) * (diff_b.z),
        z: (diff_a.x) * (diff_b.y) - (diff_a.y) * (diff_b.x),
        w: 1.,
    };
    normal
}

fn is_in_triangle(triangle: &Triangle, intersect: Vector3D, normal: Vector3D) -> bool {
    let tmp: Vector3D = calcul_normal(intersect, triangle.b, triangle.c);
    let alpha = normal.dot(tmp) / triangle.normal_sq;
    let tmp2: Vector3D = calcul_normal(intersect, triangle.c, triangle.a);
    let beta = normal.dot(tmp2) / triangle.normal_sq;
    let gamma = 1. - alpha - beta;
    let between_0_1 = 0.0..1.0;

    between_0_1.contains(&beta) && between_0_1.contains(&alpha) && between_0_1.contains(&gamma)
}

#[typetag::serde]
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
