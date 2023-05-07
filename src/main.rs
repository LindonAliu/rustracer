//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// main
//

mod transformationbuilder;
mod matrix;
mod intersection;
mod vector3d;
mod shape;
mod material;
mod camera;
mod light;
mod intersections {
    pub mod plane;
    pub mod sphere;
    pub mod triangle;
    pub mod composite;
    pub mod decorator {
        pub mod translation;
        pub mod scale;
        pub mod transformation;
    }
}
mod lights {
    pub mod point_light;
    pub mod ambient_light;
}
mod scene;

use nannou::prelude::*;
use nannou::image::ImageBuffer;
use nannou::wgpu;
use nannou::image::DynamicImage; 
use nannou::image::DynamicImage::ImageRgb8;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    framebuffer: DynamicImage,
    sizes: (u32, u32),
}

fn model(app: &App) -> Model {
    let sizes = (1920, 1080);
    let window = app.new_window().size(sizes.0, sizes.1).view(view).build().unwrap();
    let framebuffer = ImageRgb8(ImageBuffer::new(sizes.0, sizes.1));

    Model { sizes, window, framebuffer }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let texture = wgpu::Texture::from_image(
        app,
        &model.framebuffer,
    );

    draw.texture(&texture)
        .x_y(0.0, 0.0)
        .w_h(model.sizes.0 as f32, model.sizes.1 as f32);

    draw.to_frame(app, &frame).unwrap();
}
