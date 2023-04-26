//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// intersection
//

use crate::vector3d::{Point3D, Vector3D};

/// Store the result of the intersection between a ray and a shape or a light.
#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    /// The point of intersection between the ray and the shape.
    pub intersection_point: Point3D,
    /// The normal of the shape at the point of intersection.
    pub normal: Vector3D,
    /// The distance between the origin of the ray and the point of intersection.
    pub distance: f64,
}

/// Represents a ray with an origin and a direction.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    /// The origin of the ray, as a 3D point.
    pub origin: Point3D,
    /// The direction of the ray, as a 3D vector.
    pub direction: Vector3D,
}
