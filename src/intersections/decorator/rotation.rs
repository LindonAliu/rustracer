//
// EPITECH PROJECT, 2023
// rotation
// File description:
// FreeKOSOVO
//

use crate::intersections::decorator::transformation::Transformation;
use crate::transformationbuilder::TransformationBuilder;
use crate::vector3d::Point3D;
use crate::shape::Shape;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct SerRotation {
    #[serde(default)]
    pub angle_x: f64,
    #[serde(default)]
    pub angle_y: f64,
    #[serde(default)]
    pub angle_z: f64,
    pub center: Point3D,
    pub wrapped: Box<dyn Shape>,
}

#[derive(Deserialize)]
struct Rotation(Transformation);

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
