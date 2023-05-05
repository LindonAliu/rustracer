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
    // _texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let _framebuffer = Framebuffer::new(1920, 1080);
    // let _texture = wgpu::TextureBuilder::new()
    //     .size([1920, 1080])
    //     .format(wgpu::TextureFormat::Rgba8Unorm)
    //     .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
    //     .build(app.main_window().device());
    Model { _window, _framebuffer }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    draw.to_frame(app, &frame).unwrap();
}
