//
// EPITECH PROJECT, 2023
// matrix
// File description:
// FreeKOSOVO
//

use std::ops::{Add, AddAssign, Mul, Div};
use crate::vector3d::Vector3D;

pub struct Matrix {
    storage: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            storage: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn identity(size: usize) -> Matrix {
        let mut result = Matrix::new(size, size);
        for i in 0..size {
            result[(i, i)] = 1.0;
        }
        result
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (rows, cols): (usize, usize)) -> &f64 {
        &self.storage[rows * self.cols + cols]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (rows, cols): (usize, usize)) -> &mut f64 {
        &mut self.storage[rows * self.cols + cols]
    }
}

impl From<[f64; 16]> for Matrix {
    fn from(array: [f64; 16]) -> Matrix {
        let mut result = Matrix::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = array[i * 4 + j];
            }
        }
        result
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] = self[(i, j)] + other[(i, j)];
            }
        }
        result
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Matrix) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self[(i, j)] += other[(i, j)];
            }
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, other.cols);

        assert_eq!(self.cols, other.rows);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[(i, j)] += self[(i, k)] * other[(k, j)];
                }
            }
        }
        result
    }
}

impl From<Vector3D> for Matrix {
    fn from(vector: Vector3D) -> Matrix {
        let mut result = Matrix::new(4, 1);
        result[(0, 0)] = vector.x;
        result[(1, 0)] = vector.y;
        result[(2, 0)] = vector.z;
        result[(3, 0)] = vector.w;
        result
    }
}

impl From<Matrix> for Vector3D {
    fn from(matrix: Matrix) -> Vector3D {
        assert!(matrix.rows == 4 && matrix.cols == 1);
        Vector3D {
            x: matrix[(0, 0)],
            y: matrix[(1, 0)],
            z: matrix[(2, 0)],
            w: matrix[(3, 0)],
        }
    }
}

impl Mul<Vector3D> for Matrix {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        let tmp_matrix = Matrix::from(other) * self;
        let result = Vector3D::from(tmp_matrix);
        return result;
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, other: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] = self[(i, j)] / other;
            }
        }
        result
    }
}

impl Matrix {
    fn determinant(&self) -> f64 {
        let mut det = 0.0;

        match self.rows {
            1 => det = self[(0, 0)],
            2 => det = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)],
            _ => {
                for i in 0..self.rows {
                    det += self[(0, i)] * self.cofactor(0, i);
                }
            }
        }
        det
    }

    fn cofactor(&self, i: usize, j: usize) -> f64 {
        let coeff = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        let mut minor = Matrix::new(self.rows - 1, self.cols - 1);
        for rows in 0..self.rows {
            for cols in 0..self.cols {
                minor[(rows, cols)] = match (rows >= i, cols >= j) {
                    (true, true) => self[(rows + 1, cols + 1)],
                    (true, false) => self[(rows + 1, cols)],
                    (false, true) => self[(rows, cols + 1)],
                    (false, false) => self[(rows, cols)],
                }
            }
        }
        coeff * minor.determinant()
    }

    fn transposed(&self) -> Matrix {
        let mut transposed = Matrix::new(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                transposed[(col, row)] = self[(row, col)];
            }
        }
        transposed
    }

    pub fn inverse(&self) -> Matrix {
        let mut inverse = Matrix::new(self.rows, self.cols);
        let det = self.determinant();
        
        if det == 0.0 || self.rows != self.cols {
            panic!("Matrix is not invertible");
        }
        match self.rows {
            1 => inverse[(0, 0)] = 1.0 / self[(0, 0)],
            2 => {
                inverse[(0, 0)] = self[(1, 1)] / det;
                inverse[(0, 1)] = -self[(0, 1)] / det;
                inverse[(1, 0)] = -self[(1, 0)] / det;
                inverse[(1, 1)] = self[(0, 0)] / det;
            },
            _ => {
                for i in 0..self.rows {
                    for j in 0..self.cols {
                        inverse[(i, j)] = self.cofactor(i, j);
                    }
                }
                inverse = inverse.transposed() / det;
            }
        }
        inverse
    }
}
