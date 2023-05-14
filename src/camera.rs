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

fn change_element(element: &mut f64, key: &Key, sign: &str, app: &App) {
    let how_much = if app.keys.down.contains(&Key::LShift) || app.keys.down.contains(&Key::RShift) {
        1.
    } else {
        0.1
    };

    if app.keys.down.contains(key) {
        if sign == "+" {
            *element += how_much;
        } else {
            *element -= how_much;
        }
    }
}

impl Camera {   
    fn update_position(&mut self, app: &App) {
        change_element(&mut self.position.x, &Key::Left, "-", app);
        change_element(&mut self.position.x, &Key::Right, "+", app);

        change_element(&mut self.position.y, &Key::Up, "+", app);
        change_element(&mut self.position.y, &Key::Down, "-", app);

        change_element(&mut self.position.z, &Key::I, "-", app);
        change_element(&mut self.position.z, &Key::K, "+", app);
    }

    fn update_direction(&mut self, app: &App) {
        change_element(&mut self.direction.y, &Key::Z, "+", app);
        change_element(&mut self.direction.y, &Key::W, "+", app);
        change_element(&mut self.direction.y, &Key::S, "-", app);

        change_element(&mut self.direction.x, &Key::Q, "+", app);
        change_element(&mut self.direction.x, &Key::A, "+", app);
        change_element(&mut self.direction.x, &Key::D, "-", app);
    }

    pub fn update(&mut self, app: &App) {
        self.update_position(app);
        self.update_direction(app);
    }
}
