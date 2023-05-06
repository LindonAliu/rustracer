//
// EPITECH PROJECT, 2023
// transformation
// File description:
// FreeKOSOVO
//

use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::intersection::{Intersection, Ray};
use crate::vector3d::{Vector3D};
use crate::shape::Shape;

pub struct Transformation {
    [[f64; 4]; 4]
    pub transformation: Vector3D,
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

impl AddAssign for Transformation {
    fn add_assign(&mut self, other: Transformation) {
        *self = *self + other;
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

impl Mul<Vector3D> for Transformation {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        let mut result = [0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i] += self.0[i][j] * other[j];
            }
        }
        Vector3D {
            w: result[0],
            x: result[1],
            y: result[2],
            z: result[3],
        }
    }
}

impl MulAssign for Transformation {
    fn mul_assign(&mut self, other: Transformation) {
        *self = *self * other;
    }
}
