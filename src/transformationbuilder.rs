//
// EPITECH PROJECT, 2023
// transformationbuilder
// File description:
// FreeKOSOVO
//

use crate::matrix::Matrix;

pub struct TransformationBuilder {
    pub matrix: Matrix,
}

impl TransformationBuilder {
    pub fn new() -> TransformationBuilder {
        TransformationBuilder {
            matrix: Matrix::identity(4),
        }
    }

    pub fn translation(mut self, x: f64, y: f64, z: f64) -> TransformationBuilder {
        self.matrix[(0, 3)] = x;
        self.matrix[(1, 3)] = y;
        self.matrix[(2, 3)] = z;
        self
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> TransformationBuilder {
        self.matrix[(0, 0)] = x;
        self.matrix[(1, 1)] = y;
        self.matrix[(2, 2)] = z;
        self
    }

    pub fn rotation_x(mut self, radians: f64) -> TransformationBuilder {
        self.matrix[(1, 1)] = radians.cos();
        self.matrix[(1, 2)] = -radians.sin();
        self.matrix[(2, 1)] = radians.sin();
        self.matrix[(2, 2)] = radians.cos();
        self
    }

    pub fn rotation_y(mut self, radians: f64) -> TransformationBuilder {
        self.matrix[(0, 0)] = radians.cos();
        self.matrix[(0, 2)] = radians.sin();
        self.matrix[(2, 0)] = -radians.sin();
        self.matrix[(2, 2)] = radians.cos();
        self
    }

    pub fn rotation_z(mut self, radians: f64) -> TransformationBuilder {
        self.matrix[(0, 0)] = radians.cos();
        self.matrix[(0, 1)] = -radians.sin();
        self.matrix[(1, 0)] = radians.sin();
        self.matrix[(1, 1)] = radians.cos();
        self
    }

    pub fn get_matrix(self) -> Matrix {
        self.matrix
    }
}
