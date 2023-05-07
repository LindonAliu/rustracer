//
// EPITECH PROJECT, 2023
// transformation
// File description:
// FreeKOSOVO
//

use crate::intersection::{Intersection, Ray};
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

#[typetag::serde]
impl Shape for Transformation {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let new_origin = self.reverse_transformation * ray.origin;
        let scaled_ray = Ray {
            origin: new_origin,
            direction: (self.reverse_transformation *
                (ray.origin + ray.direction)) - new_origin,
        };

        if let Some(intersection) = self.wrapped.intersect(&scaled_ray) {
            let mut new_intersection = self.transformation * intersection.intersection_point;
            new_intersection.w = 1.0;
            let mut distance = (ray.origin - new_intersection).length();
            distance.w = 1.0;

            let mut normal = self.transformation *
                (intersection.intersection_point + intersection.normal) -
                new_intersection;
            normal.w = 1.0;

            Some(Intersection {
                intersection_point: new_intersection,
                normal: normal,
                distance: distance,
                shape: self,
            })
        } else {
            None
        }
    }
}
