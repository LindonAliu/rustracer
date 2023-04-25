//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// intersection
//

mod vector3d;

/// Intersection struct
///
/// This struct contains the intersection point, the normal and the distance
/// between the intersection point and the origin of the ray.
///
/// It is used to store the result of the intersection between a ray and
/// a shape or a light.
#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub intersection_point: Point3D,
    pub normal: Vector3D,
    pub distance: f64,
}

/// Ray struct
///
/// This struct contains the origin and the direction of a ray.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}
