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
    pub matrix: [[f64; 4]; 4],
    pub wrapped: Box<dyn Shape>,
}

impl Add for Transformation {
    type Output = Transformation;

    fn add(self, other: Transformation) -> Transformation {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.matrix[i][j] + other.matrix[i][j];
            }
        }
        Transformation {
            matrix: result,
            wrapped: self.wrapped,
        }
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
                    result[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        Transformation {
            matrix: result,
            wrapped: self.wrapped,
        }
    }
}

impl Mul<Vector3D> for Transformation {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        let mut result = [0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i] += self.matrix[i][j] * other[j];
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

impl Shape for Transformation {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    }
}
