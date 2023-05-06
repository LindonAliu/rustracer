//
// EPITECH PROJECT, 2023
// rotation
// File description:
// FreeKOSOVO
//

use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::intersection::{Intersection, Ray};
use crate::vector3d::{Vector3D};
use crate::shape::Shape;

pub struct Rotation {
    pub wrapped: Box<dyn Shape>,
    pub rotation: Vector3D,
}