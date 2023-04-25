//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// shape
//

mod intersection;
use intersection::{Intersection, Ray};

/// Shape trait
///
/// This trait is used to define a shape.
pub trait Shape {
    /// Intersect a ray with a shape
    ///
    /// This function returns an Intersection if the ray intersects the shape.
    /// If the ray doesn't intersect the shape, it returns None.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    /// Get the color of a shape
    fn getColor(&self) -> Color;
    /// Get the material of a shape
    fn getMaterial(&self) -> Material;
}
