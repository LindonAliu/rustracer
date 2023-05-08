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
    position: Point3D,
    direction: Vector3D,
    width: u32,
    height: u32,
    fov: u32,
}
