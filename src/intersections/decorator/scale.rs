//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// scale shape
//

use crate::intersection::{Intersection, Ray};
use crate::vector3d::{Vector3D};
use crate::shape::Shape;

pub struct Scale {
    pub wrapped: Box<dyn Shape>,
    pub scale: Vector3D,
}

impl Shape for Scale {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let scaled_ray = Ray {
            origin: ray.origin / self.scale,
            direction: ray.direction,
        };
        if let Some(intersection) = self.wrapped.intersect(&scaled_ray) {
            let mut intersection = intersection;
            intersection.intersection_point *= self.scale;
            Some(intersection)
        } else {
            None
        }
    }
}
