// raytracer
// File description:
// scene
//

use crate::shape::Shape;
use serde::{Serialize, Deserialize};
use crate::camera::Camera;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shape: Box<dyn Shape>,
    pub cam: Camera,
}

