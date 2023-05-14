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

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shape: Box<dyn Shape>,
    pub camera: Camera,
    pub lights: Vec<Box<dyn Light>>
}

fn update_transformation_builder(app: &App) -> Matrix {
    let mut tb = TransformationBuilder::new();
    let how_much = if app.keys.down.contains(&Key::LShift) || app.keys.down.contains(&Key::RShift) {
        1.
    } else {
        0.1
    };

    if app.keys.down.contains(&Key::Q) || app.keys.down.contains(&Key::A) {
        tb = tb.rotation_y(how_much);
    }
    if app.keys.down.contains(&Key::D) {
        tb = tb.rotation_y(-how_much);
    }
    if app.keys.down.contains(&Key::W) || app.keys.down.contains(&Key::Z) {
        tb = tb.rotation_x(how_much);
    }
    if app.keys.down.contains(&Key::S) {
        tb = tb.rotation_x(-how_much);
    }
    tb.get_matrix()
}

impl Scene {
    pub fn update(&mut self, app: &App) {
        let matrix = get_camera_transformation(self);
        let transformation_matrix = update_transformation_builder(app);

        self.camera.direction = matrix.clone() * transformation_matrix
            * matrix.inverse() * self.camera.direction;
    }
}
