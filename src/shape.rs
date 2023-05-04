//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// shape
//

use crate::intersection::{Intersection, Ray};

/// This trait is used to define any 3D shape.
#[typetag::serde(tag = "type")]
pub trait Shape {
    /// This function returns an Intersection if the ray intersects the shape.
    /// If the ray doesn't intersect the shape, it returns None.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
