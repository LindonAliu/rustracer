//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// directional light
//

use crate::material::Color;
use crate::vector3d::Vector3D;
use crate::light::Light;
use crate::intersection::Intersection;
use crate::shape::Shape;

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Directional {
    pub direction: Vector3D,
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
    pub color: Color
}

fn default_multiplier() -> f64 {
    1.
}

#[typetag::serde]
impl Light for Directional {
    fn light(&self, intersection: &Intersection, color: &Color, _shape: &dyn Shape) -> Color {
        let l = self.direction;
        let cos_a: f64 = intersection.normal.dot(l) / (intersection.normal.length() * l.length());
        let multiplier: f64 = if cos_a > 0. {
            cos_a * self.multiplier
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
