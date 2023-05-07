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

impl Mul<Vector3D> for Matrix {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        let mut result = [0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i] += self[(i, j)] * other[j];
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
        let mut transposed = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed[(i, j)] = self[(j, i)];
            }
        }
        transposed
    }

    fn inverse(&self) -> Matrix {
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
