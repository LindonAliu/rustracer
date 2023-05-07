//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// translation shape
//

use crate::intersection::{Intersection, Ray};
use crate::vector3d::Vector3D;
use crate::shape::Shape;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Translation {
    pub wrapped: Box<dyn Shape>,
    pub translation: Vector3D,
}

#[typetag::serde]
impl Shape for Translation {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let translated_ray = Ray {
            origin: ray.origin - self.translation,
            direction: ray.direction,
        };
        if let Some(intersection) = self.wrapped.intersect(&translated_ray) {
            let mut intersection = intersection;
            intersection.intersection_point += self.translation;
            Some(intersection)
        } else {
            None
        }
    }
}
