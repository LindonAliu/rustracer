//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// shape
//

use crate::intersection::{Intersection, Ray};

/// Contains the RGBA values of a color.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Contains the different materials that can be used.
#[derive(Copy, Clone, Debug)]
pub enum Material {
    /// Represents a color, with optional transparency.
    Color(Color),
    /// Represents a mirror.
    Mirror,
}

/// This trait is used to define any 3D shape.
pub trait Shape {
    /// This function returns an Intersection if the ray intersects the shape.
    /// If the ray doesn't intersect the shape, it returns None.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    /// Get the material of a shape
    fn material(&self) -> &Material;
}
