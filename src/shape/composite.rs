//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// composite shape
//

use crate::intersection::{Intersection, Ray};
use crate::shape::Shape;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Composite {
    pub shapes: Vec<Box<dyn Shape>>
}

#[typetag::serde]
impl Shape for Composite {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;
        for shape in &self.shapes {
            match (shape.intersect(ray), closest) {
                (Some(intersection), None) => {
                    closest = Some(intersection);
                },
                (Some(intersection), Some(prev_closest))
                    if intersection.distance < prev_closest.distance => {
                    closest = Some(intersection);
                },
                _ => (),
            }
        }
        closest
    }
}
