// raytracer
// File description:
// scene
//

use crate::shape::Shape;
use serde::{Serialize, Deserialize};
use crate::camera::Camera;
use crate::light::Light;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shape: Box<dyn Shape>,
    pub camera: Camera,
    pub lights: Vec<Box<dyn Light>>
}

