//
// EPITECH PROJECT, 2023
// transformation
// File description:
// FreeKOSOVO
//

use crate::intersection::{Intersection, Ray};
use crate::matrix::Matrix;
use crate::shape::Shape;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SerTransformation {
    transformation: [f64; 16],
    wrapped: Box<dyn Shape>,
}

#[derive(Serialize, Deserialize)]
#[serde(from = "SerTransformation")]
pub struct Transformation {
    #[serde(skip)]
    pub transformation: Matrix,
    #[serde(skip)]
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
        let new_origin = self.reverse_transformation.clone() * ray.origin;
        let scaled_ray = Ray {
            origin: new_origin,
            direction: (self.reverse_transformation.clone() *
                (ray.origin + ray.direction)) - new_origin,
        };
        
        if let Some(intersection) = self.wrapped.intersect(&scaled_ray) {
            let new_intersection = self.transformation.clone() * intersection.intersection_point;
            let distance = (ray.origin - new_intersection).length();
            let mut normal = self.transformation.clone() *
                (intersection.intersection_point + intersection.normal) - new_intersection;

            normal.w = 1.0;
            Some(Intersection {
                intersection_point: new_intersection,
                normal: normal,
                distance: distance,
                material: intersection.material,
            })
        } else {
            None
        }
    }
}
