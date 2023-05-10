//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// scale shape
//

use crate::intersection::{Intersection, Ray};
use crate::vector3d::Vector3D;
use crate::shape::Shape;
use crate::shape::decorator::Transformation;
use crate::transformationbuilder::TransformationBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SerScale {
    wrapped: Box<dyn Shape>,
    scale: Vector3D,
    center: Vector3D,
}

#[derive(Serialize, Deserialize)]
#[serde(from = "SerScale")]
pub struct Scale(Transformation);

impl From<SerScale> for Scale {
    fn from(other: SerScale) -> Self {
        let transformation = TransformationBuilder::new()
            .translation(-other.center)
            .scale(other.scale)
            .translation(other.center)
            .get_matrix();
        let reverse_transformation = transformation.inverse();
        Scale(Transformation {
            transformation,
            reverse_transformation,
            wrapped: other.wrapped,
        })
    }
}

#[typetag::serde]
impl Shape for Scale {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.0.intersect(ray)
    }
}
