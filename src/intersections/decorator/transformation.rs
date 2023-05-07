//
// EPITECH PROJECT, 2023
// transformation
// File description:
// FreeKOSOVO
//

use crate::matrix::Matrix;
use crate::shape::Shape;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct SerTransformation {
    pub transformation: [f64; 16],
    pub wrapped: Box<dyn Shape>,
}

#[derive(Deserialize)]
#[serde(from = "SerTransformation")]
pub struct Transformation {
    pub transformation: Matrix,
    pub reverse_transformation: Matrix,
    pub wrapped: Box<dyn Shape>,
}

impl From<SerTransformation> for Transformation {
    fn from(other: SerTransformation) -> Self {
        Transformation {
            transformation: Matrix::from(other.transformation),
            reverse_transformation: Matrix::from(other.transformation).inverse(),
            wrapped: other.wrapped,
        }
    }
}
