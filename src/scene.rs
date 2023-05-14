// raytracer
// File description:
// scene
//

use crate::shape::Shape;
use serde::{Serialize, Deserialize};
use crate::camera::Camera;
use crate::light::Light;
use crate::trace_rays::get_camera_transformation;
use crate::transformationbuilder::TransformationBuilder;
use nannou::App;
use nannou::event::Key;
use crate::matrix::Matrix;
use crate::vector3d::Vector3D;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shape: Box<dyn Shape>,
    pub camera: Camera,
    pub lights: Vec<Box<dyn Light>>
}

fn get_matrix_direction_event_y(app: &App) -> Matrix {
    let mut tb = TransformationBuilder::new();
    let how_much = if app.keys.down.contains(&Key::LShift) || app.keys.down.contains(&Key::RShift) {
        1.
    } else {
        0.1
    };

    if app.keys.down.contains(&Key::Q) || app.keys.down.contains(&Key::A) {
        tb = tb.rotation_y(-how_much);
    }
    if app.keys.down.contains(&Key::D) {
        tb = tb.rotation_y(how_much);
    }
    tb.get_matrix()
}

fn get_matrix_direction_event_x(app: &App) -> Matrix {
    let mut tb = TransformationBuilder::new();
    let how_much = if app.keys.down.contains(&Key::LShift) || app.keys.down.contains(&Key::RShift) {
        1.
    } else {
        0.1
    };

    if app.keys.down.contains(&Key::W) || app.keys.down.contains(&Key::Z) {
        tb = tb.rotation_x(how_much);
    }
    if app.keys.down.contains(&Key::S) {
        tb = tb.rotation_x(-how_much);
    }
    tb.get_matrix()
}

fn get_position_matrix_event(app: &App) -> Vector3D {
    let mut tb = Vector3D { x: 0., y: 0., z: 0., w: 1.};
    let how_much = if app.keys.down.contains(&Key::LShift) || app.keys.down.contains(&Key::RShift) {
        5.
    } else {
        1.
    };

    if app.keys.down.contains(&Key::Left) {
        tb.x -= how_much;
    }
    if app.keys.down.contains(&Key::Right) {
        tb.x += how_much;
    }
    if app.keys.down.contains(&Key::Up) {
        tb.y += how_much;
    }
    if app.keys.down.contains(&Key::Down) {
        tb.y -= how_much;
    }
    if app.keys.down.contains(&Key::I) {
        tb.z += how_much;
    }
    if app.keys.down.contains(&Key::K) {
        tb.z -= how_much;
    }
    tb
}

impl Scene {

    pub fn update_direction(&mut self, app: &App) {
        let matrix_direction = get_camera_transformation(self);
        let event_x = get_matrix_direction_event_x(app);
        let event_y = get_matrix_direction_event_y(app);

        let direction = event_y * matrix_direction.clone() * event_x;

        self.camera.direction.x = direction[(2, 0)];
        self.camera.direction.y = direction[(2, 1)];
        self.camera.direction.z = direction[(2, 2)];

    }

    pub fn update_position(&mut self, app: &App) {
        let matrix = get_camera_transformation(self);
        let translations = get_position_matrix_event(app);

        self.camera.position += matrix.clone() * translations;
    }

    pub fn update(&mut self, app: &App) {
        self.update_direction(app);
        self.update_position(app);
    }
}
