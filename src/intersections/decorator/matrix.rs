//
// EPITECH PROJECT, 2023
// matrix
// File description:
// FreeKOSOVO
//

use std::ops::{Add, AddAssign, Mul, MulAssign};

pub struct matrix {
    pub matrix: Vec<Vec<f64>>,
    usize: row,
    usize: col,
}

impl Add for matrix {
    type Output = matrix;

    fn add(self, other: matrix) -> matrix {
        let mut result = matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in 0..self.col {
                result.matrix[i][j] = self.matrix[i][j] + other.matrix[i][j];
            }
        }
        result
    }
}

impl AddAssign for matrix {
    fn add_assign(&mut self, other: matrix) {
        *self = *self + other;
    }
}

impl Mul for matrix {
    type Output = matrix;

    fn mul(self, other: matrix) -> matrix {
        let mut result = matrix::new(self.row, other.col);
        for i in 0..self.row {
            for j in 0..other.col {
                for k in 0..self.col {
                    result.matrix[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        result
    }
}

impl Mul<f64> for matrix {
    type Output = matrix;

    fn mul(self, other: f64) -> matrix {
        let mut result = matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in 0..self.col {
                result.matrix[i][j] = self.matrix[i][j] * other;
            }
        }
        result
    }
}

impl matrix {
    fn new(usize: row, usize: col) -> matrix {
        let mut matrix = Vec::new();
        for i in 0..row {
            let mut row = Vec::new();
            for j in 0..col {
                row.push(0.0);
            }
            matrix.push(row);
        }
        matrix {
            matrix: matrix,
            row: row,
            col: col,
        }
    }

    fn cofactor(&self, i: usize, j: usize) -> f64 {
        let coeff = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        let mut minor = matrix::new(self.row - 1, self.col - 1);
        for row in 0..self.matrix.row {
            for col in 0..self.matrix.col {
                minor.matrix[row][col] = self.matrix[
                    if row >= i { row + 1 } else { row }
                ][
                    if col >= j { col + 1 } else { col }
                ]
            }
        }
        coeff * minor.determinant()
    }

    fn determinant(&self) -> f64 {
        let mut det = 0.0;

        match self.row {
            1 => det = self.matrix[0][0],
            2 => det = self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0],
            _ => {
                for i in 0..self.row {
                    det += self.matrix[0][i] * self.cofactor(0, i);
                }
            }
        }
        det
    }

    fn transposed(&self) -> matrix {
        let mut transposed = matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in 0..self.col {
                transposed.matrix[i][j] = self.matrix[j][i];
            }
        }
        transposed
    }

    fn inverse(&self) -> matrix {
        let mut inverse = matrix::new(self.row, self.col);
        let det = self.determinant();

        match self.row {
            1 => inverse.matrix[0][0] = 1.0 / self.matrix[0][0],
            2 => {
                if det == 0.0 {
                    panic!("Matrix is not invertible");
                }
                inverse.matrix[0][0] = self.matrix[1][1] / det;
                inverse.matrix[0][1] = -self.matrix[0][1] / det;
                inverse.matrix[1][0] = -self.matrix[1][0] / det;
                inverse.matrix[1][1] = self.matrix[0][0] / det;
            },
            _ => {
                for row in 0..self.row {
                    for col in 0..self.col {
                        inverse.matrix[row][col] = self.cofactor(row, col);
                    }
                }
                if det == 0.0 {
                    panic!("Matrix is not invertible");
                }
                inverse = inverse.transposed() / det;
            }
        }
        inverse
    }
}
