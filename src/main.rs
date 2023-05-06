//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// main
//

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
    if std::env::args().len() != 2 {
        println!("Usage: ./raytracer <scene.json>");
        std::process::exit(84);
    }
    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    framebuffer: DynamicImage,
    sizes: (u32, u32),
    scene: Scene,
}

fn model(app: &App) -> Model {
    let sizes = (1920, 1080);
    let window = app.new_window().size(sizes.0, sizes.1).view(view).build().unwrap();
    let framebuffer = ImageRgb8(ImageBuffer::new(sizes.0, sizes.1));
    let filename = std::env::args().nth(1).unwrap();
    if let Ok(file) = std::fs::read_to_string(filename) {
        let scene = serde_json::from_str::<Scene>(file.as_str());
        match scene {
            Ok(scene) => Model { sizes, window, framebuffer, scene },
            Err(e) => {
                println!("Error: cannot parse scene file: {}", e);
                std::process::exit(84);
            }
        }
    } else {
        println!("Error: cannot open scene file");
        std::process::exit(84);
    }
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
