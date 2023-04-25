//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// shape
//

use crate::intersection::{Intersection, Ray};

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub enum Material {
    Color(Color),
    Mirror,
}

/// Shape trait
///
/// This trait is used to define a shape.
pub trait Shape {
    /// Intersect a ray with a shape
    ///
    /// This function returns an Intersection if the ray intersects the shape.
    /// If the ray doesn't intersect the shape, it returns None.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    /// Get the material of a shape
    fn material(&self) -> &Material;
}
