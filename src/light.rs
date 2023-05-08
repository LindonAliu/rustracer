//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// light
//

pub mod point;
pub mod ambient;

use crate::material::Color;
use crate::intersection::{Intersection};

pub trait Light {
    fn light(&self, intersection: &Intersection, color: &Color) -> Color;
}
