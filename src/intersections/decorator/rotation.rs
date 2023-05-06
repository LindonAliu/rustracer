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

pub struct Transformation {
    [[f64; 4]; 4]
}

impl Add for Transformation {
    type Output = Transformation;

    fn add(self, other: Transformation) -> Transformation {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        Transformation(result)
    }
}

impl Mul for Transformation {
    type Output = Transformation;

    fn mul(self, other: Transformation) -> Transformation {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        Transformation(result)
    }
}
