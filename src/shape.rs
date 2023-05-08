//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// shape
//

mod sphere;
pub use sphere::Sphere;
mod plane;
pub use plane::Plane;
mod triangle;
pub use triangle::Triangle;
mod composite;
pub use composite::Composite;
pub mod decorator {
    mod translation;
    pub use translation::Translation;
    mod scale;
    pub use scale::Scale;
    mod rotation;
    pub use rotation::Rotation;
    mod transformation;
    pub use transformation::Transformation;
}

use crate::intersection::{Intersection, Ray};

/// This trait is used to define any 3D shape.
#[typetag::serde(tag = "type")]
pub trait Shape {
    /// This function returns an Intersection if the ray intersects the shape.
    /// If the ray doesn't intersect the shape, it returns None.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
