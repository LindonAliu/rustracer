//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// include another scene
//

use crate::intersection::{Intersection, Ray};
use crate::shape::Shape;
use crate::scene::Scene;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SerInclude {
    path: String
}

#[derive(Serialize, Deserialize)]
#[serde(try_from = "SerInclude")]
pub struct Include {
    pub wrapped: Box<dyn Shape>,
}

impl TryFrom<SerInclude> for Include {
    type Error = std::io::Error;

    fn try_from(ser: SerInclude) -> Result<Self, Self::Error> {
        let file = std::fs::read_to_string(ser.path)?;
        let scene = serde_json::from_str::<Scene>(&file)?;
        Ok(Include { wrapped: scene.shape })
    }
}

#[typetag::serde]
impl Shape for Include {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.wrapped.intersect(ray)
    }
}
