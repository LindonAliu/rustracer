//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// shape
//

use crate::intersection::{Intersection, Ray};

/// Color struct
///
/// This struct contains the RGBA values of a color.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Material enum
///
/// This enum contains the different materials that can be used.
#[derive(Copy, Clone, Debug)]
pub enum Material {
    /// Color material
    ///
    /// This material represents a color.
    Color(Color),
    /// Mirror material
    ///
    /// This material represents a mirror.
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
