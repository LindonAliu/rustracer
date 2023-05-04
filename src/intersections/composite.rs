//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// composite shape
//

use crate::intersection::{Intersection, Ray};
use crate::shape::Shape;

pub struct CompositeShape {
    pub shapes: Vec<Box<dyn Shape>>
}

impl Shape for CompositeShape {
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
