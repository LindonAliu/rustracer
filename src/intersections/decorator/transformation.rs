//
// EPITECH PROJECT, 2023
// transformation
// File description:
// FreeKOSOVO
//

use crate::vector3d::{Vector3D};
use crate::matrix::Matrix;

pub struct Transformation {
    pub matrix: Matrix,
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
