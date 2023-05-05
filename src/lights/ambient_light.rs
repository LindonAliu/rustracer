//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// ambient_light
//

use crate::material::Color;
use crate::light::Light;
use crate::intersection::{Intersection};

#[derive(Copy, Clone, Debug)]
pub struct AmbientLight {
    pub multiplier: f64,
    pub color: Color
}

impl Light for AmbientLight {
    fn light(&self, _intersection: &Intersection, color: &Color) -> Color {
        let multiplier: f64 = self.multiplier;

        Color {
            r: ((self.color.r as f64 / 255.0) * multiplier * (color.r as f64)) as u8,
            g: ((self.color.g as f64 / 255.0) * multiplier * (color.g as f64)) as u8,
            b: ((self.color.b as f64 / 255.0) * multiplier * (color.b as f64)) as u8,
            a: color.a
        }
    }
}
