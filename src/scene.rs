// raytracer
// File description:
// scene
//

use crate::shape::Shape;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shape: Box<dyn Shape>
}

