//
// EPITECH PROJECT, 2023
// rotation
// File description:
// FreeKOSOVO
//

use crate::shape::decorator::Transformation;
use crate::transformationbuilder::TransformationBuilder;
use crate::intersection::{Intersection, Ray};
use crate::vector3d::Point3D;
use crate::shape::Shape;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SerRotation {
    #[serde(default)]
    angle_x: f64,
    #[serde(default)]
    angle_y: f64,
    #[serde(default)]
    angle_z: f64,
    center: Point3D,
    wrapped: Box<dyn Shape>,
}

#[derive(Serialize, Deserialize)]
#[serde(from = "SerRotation")]
pub struct Rotation(Transformation);

impl From<SerRotation> for Rotation {
    fn from(other: SerRotation) -> Self {
        let transformation = TransformationBuilder::new()
            .translation(-other.center)
            .rotation_x(other.angle_x.to_radians())
            .rotation_y(other.angle_y.to_radians())
            .rotation_z(other.angle_z.to_radians())
            .translation(other.center)
            .get_matrix();
        let reverse_transformation = transformation.inverse();
        Rotation(Transformation {
            transformation,
            reverse_transformation,
            wrapped: other.wrapped,
        })
    }
}

#[typetag::serde]
impl Shape for Rotation {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.0.intersect(ray)
    }
}
