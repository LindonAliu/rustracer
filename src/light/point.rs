//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// point light
//

use crate::material::Color;
use crate::vector3d::{Point3D, Vector3D};
use crate::light::Light;
use crate::intersection::{Intersection, Ray};
use crate::shape::Shape;
use crate::material::Material;

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub pos: Point3D,
    pub color: Color
}

impl Point {
    fn intersect(&self, intersection: &Intersection, shape: &dyn Shape) -> Option<Intersection> {
        let l: Vector3D = self.pos - intersection.intersection_point;
        let lightray: Ray = Ray {
            origin: intersection.intersection_point + l * 1e-6,
            direction: l,
        };

        shape.intersect(&lightray)
    }
    fn shadow(&self, intersection: &Intersection, shape: &dyn Shape) -> f64 {
        let distance: f64 = (self.pos - intersection.intersection_point).length();
        if let Some (result) = self.intersect(intersection, shape) {
            if result.distance > distance - 1e-06 {
                1.
            } else if let Material::Color(color) = result.material {
                if color.a == 255 {
                    0.
                } else {
                    self.shadow(&result, shape) * (((255. - color.a as f64) / 255.))
                }
            } else {
                0. // TODO: mirrrrrooorr
            }
        } else {
            1.
        }

    }
}

#[typetag::serde]
impl Light for Point {
    fn light(&self, intersection: &Intersection, color: &Color, shape: &dyn Shape) -> Color {
        let l: Vector3D = self.pos - intersection.intersection_point;
        let cos_a: f64 = intersection.normal.dot(l) / (intersection.normal.length() * l.length());
        let multiplier: f64 = if cos_a > 0. {
            cos_a * self.shadow(intersection, shape)
        } else {
            0.
        };

        Color {
            r: ((self.color.r as f64 / 255.0) * multiplier * (color.r as f64)) as u8,
            g: ((self.color.g as f64 / 255.0) * multiplier * (color.g as f64)) as u8,
            b: ((self.color.b as f64 / 255.0) * multiplier * (color.b as f64)) as u8,
            a: color.a
        }
    }
}
