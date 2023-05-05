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
use nannou::wgpu::Device;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    framebuffer: Framebuffer,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let width = 1920;
    let height = 1080;
    let window = app.new_window().size(width, height).view(view).build().unwrap();
    let framebuffer = Framebuffer::new(width as usize, height as usize);
    let texture = wgpu::TextureBuilder::new()
        .size([width, height])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .build(app.main_window().device());
    Model { window, framebuffer, texture } 
}

fn update(_app: &App, model: &mut Model, _update: Update) {
}

fn colors_to_u8_array(colors: &Vec<Color>) -> Vec<u8> {
    let mut u8_colors = Vec::new();
    for color in colors {
        u8_colors.push(color.r);
        u8_colors.push(color.g);
        u8_colors.push(color.b);
        u8_colors.push(color.a);
    }
    u8_colors
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let framebuffer = &model.framebuffer;
    let texture = &model.texture;

    texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &colors_to_u8_array(&framebuffer.colors),
    );

    draw.texture(texture)
        .x_y(0.0, 0.0)
        .w_h(framebuffer.width as f32, framebuffer.height as f32);

    draw.to_frame(app, &frame).unwrap();
}
