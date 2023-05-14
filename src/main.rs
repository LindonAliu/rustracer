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
mod scene;
mod trace_rays;

use nannou::prelude::*;
use nannou::image::ImageBuffer;
use nannou::wgpu;
use nannou::image::DynamicImage;
use nannou::image::DynamicImage::ImageRgb8;

use crate::scene::Scene;

fn main() {
    let argc = std::env::args().len();
    if !(2..=3).contains(&argc) {
        println!("Usage: ./raytracer <scene.json> [output.png]");
        std::process::exit(84);
    }
    nannou::app(model).update(update).run();
}

struct Model {
    framebuffer: DynamicImage,
    sizes: (u32, u32),
    scene: Scene,
}

fn model(app: &App) -> Model {
    let filename = std::env::args().nth(1).unwrap();
    let scene_result = std::fs::read_to_string(filename)
      .and_then(|file| serde_json::from_str::<Scene>(&file).map_err(Into::into));

    match scene_result {
        Ok(scene) => {
            let sizes = (scene.camera.width, scene.camera.height);
            let framebuffer = ImageRgb8(ImageBuffer::new(sizes.0, sizes.1));
            app.new_window()
                .size(sizes.0, sizes.1)
                .view(view)
                .build()
                .unwrap();

            Model { sizes, framebuffer, scene }
        }
        Err(e) => {
            eprintln!("Error: cannot parse scene file: {}", e);
            std::process::exit(84);
        }
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.scene.update(_app);
    trace_rays::trace_rays(&_model.scene, &mut _model.framebuffer);
    if let Some(filename) = std::env::args().nth(2) {
        if let Err(err) = _model.framebuffer.save(filename) {
            eprintln!("Error: cannot save image: {}", err);
            std::process::exit(84);
        }
    }
    println!("Done!");
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
