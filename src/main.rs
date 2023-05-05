//
// EPITECH PROJECT, 2023
// main
// File description:
// FreeKOSOVO
//

mod intersection;
mod vector3d;
mod shape;
mod material;
mod camera;
mod framebuffer;
mod intersections {
    pub mod plane;
    pub mod sphere;
    pub mod triangle;
    pub mod composite;
    pub mod decorator {
        pub mod translation;
        pub mod scale;
    }
}
mod scene;
use intersections::plane::Plane;

use vector3d::Vector3D;
use vector3d::Point3D;
use crate::intersection::Ray;
use material::Material;
use material::Color;
use scene::Scene;
use framebuffer::Framebuffer;

use nannou::prelude::*;
use nannou::image::ImageBuffer;
use nannou::wgpu;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    _framebuffer: Framebuffer,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let _framebuffer = Framebuffer::new(1920, 1080);
    Model { _window, _framebuffer } 
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let framebuffer = &_model._framebuffer;
    let width = framebuffer.width;
    let height = framebuffer.height;

    // draw.background().color(PLUM);

    for x in 0..width {
        for y in 0..height {
            let color = framebuffer.get(x, y);
            let x_window = x as f32 - (width as f32 / 2.0);
            let y_window = y as f32 - (height as f32 / 2.0);

            draw.rect()
                .x_y(x_window, y_window)
                .w_h(1.0, 1.0)
                .color(rgba(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                    color.a as f32 / 255.0,
                ));
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
