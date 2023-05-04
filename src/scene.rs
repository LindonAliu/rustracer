// raytracer
// File description:
// scene
//

use crate::shape::Shape;

pub struct Scene {
    pub shape: Box<dyn Shape>
}

