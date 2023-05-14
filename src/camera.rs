//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// camera
//

use crate::vector3d::{Point3D, Vector3D};
use serde::{Deserialize, Serialize};
use nannou::prelude::*;

/// Represents a camera.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Point3D,
    pub direction: Vector3D,
    pub width: u32,
    pub height: u32,
    pub fov: f64,
}

impl Camera {
    fn update_position(&mut self, app: &App) {
        if app.keys.down.contains(&Key::Left) {
            self.position.x -= 0.1;
        }
        if app.keys.down.contains(&Key::Right) {
            self.position.x += 0.1;
        }
        if app.keys.down.contains(&Key::Up) {
            self.position.y -= 0.1;
        }
        if app.keys.down.contains(&Key::Down) {
            self.position.y += 0.1;
        }
        if app.keys.down.contains(&Key::A) {
            self.position.z -= 0.1;
        }
        if app.keys.down.contains(&Key::E) {
            self.position.z += 0.1;
        }
    }

    fn update_direction(&mut self, app: &App) {
        if app.keys.down.contains(&Key::W) {
            self.direction.x -= 0.1;
        }
        if app.keys.down.contains(&Key::E) {
            self.direction.x += 0.1;
        }
        if app.keys.down.contains(&Key::S) {
            self.direction.y -= 0.1;
        }
        if app.keys.down.contains(&Key::N) {
            self.direction.y += 0.1;
        }
    }

    pub fn update(&mut self, app: &App) {
        self.update_position(app);
        self.update_direction(app);
    }
}
