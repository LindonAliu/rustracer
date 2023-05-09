//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// camera
//

use crate::vector3d::{Point3D, Vector3D};
use serde::{Deserialize, Serialize};

/// Represents a camera.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Point3D,
    pub direction: Vector3D,
    pub width: u32,
    pub height: u32,
    pub fov: f64,
}
