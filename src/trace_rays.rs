//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// trace_rays
//

use crate::scene::Scene;
use nannou::image::DynamicImage;
use nannou::image::RgbImage;
use nannou::image::Rgb;
use crate::intersection::Ray;
use crate::vector3d::{Point3D, Vector3D};
use crate::material::Color;
use crate::material::Material;
use crate::shape::Shape;
use crate::light::Light;
use crate::intersection::{Intersection};

pub struct InfColor {
    r: f64,
    g: f64,
    b: f64
}

fn correct_gamma(color: InfColor) -> Color {
    Color {
        r : ((1. - 1. / (2. * color.r / 255. + 1.)) * 255.) as u8,
        g : ((1. - 1. / (2. * color.g / 255. + 1.)) * 255.) as u8,
        b : ((1. - 1. / (2. * color.b / 255. + 1.)) * 255.) as u8,
        a: 255
    }
}

fn modify_lights(color: &Color, lights: Vec<Box<dyn Light>>, intersection: &Intersection, shape: &dyn Shape) -> Color {
    let mut infcolor: InfColor = InfColor {
        r: 0.,
        g: 0.,
        b: 0.
    };

    for light in lights {
        let color = light.light(intersection, color, shape);
        infcolor.r += color.r as f64;
        infcolor.g += color.g as f64;
        infcolor.b += color.b as f64;
    }
    correct_gamma(infcolor)
}

pub fn trace_ray(ray: &Ray, shape: &dyn Shape, lights: Vec<Box<dyn Light>>) -> Color {
    match shape.intersect(ray) {
        None => Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        },
        Some (result) => {
            match result.material {
                Material::Color(color) => {
                    if color.a == 255 {
                        return modify_lights(&color, lights, &result, shape);
                    }
                    // let res: Intersection = Intersection {
                    //     intersection_point: result.intersection_point,
                    //     normal: ray.direction,
                    //     distance: result.distance,
                    //     material: result.material
                    // };
                    let mut new_color = modify_lights(&color, lights, &result, shape);
                    new_color.a = color.a;
                    color
                },
                Material::Mirror => {
                    Color {
                        r: 1,
                        g: 0,
                        b: 0,
                        a: 0
                    }
                }
            }
        }
    }
}

pub fn trace_rays(scene: &Scene, framebuffer: &mut DynamicImage) {
    let buffer: &mut RgbImage = framebuffer.as_mut_rgb8().unwrap();
    let mut ray: Ray = Ray {
        origin: Point3D {x: 0.0, y: 0.0, z:0.0, w:0.0},
        direction: Vector3D { x: (0.), y: (0.0), z: (500.0), w: (0.0) }
    };
    let width = scene.camera.width as f64;
    let height = scene.camera.height as f64;
    let tan_a = (scene.camera.fov / 2.0).tan();
    let tan_b = tan_a * (width / height);
    let delta = (2.0 * tan_b) / width;

    ray.direction.z = 1.0;
    for x in 0..scene.camera.width {
        ray.direction.x = -tan_b + (x as f64 * delta);
        for y in 0..scene.camera.height {
            ray.direction.y = -tan_a + (y as f64 * delta);
            match scene.shape.intersect(&ray) {
                Some(intersection) => {
                    let color: Rgb<u8> =
                        match intersection.material {
                            Material::Color(Color{r, g, b, ..}) => {
                                [r, g, b].into()
                            },
                            _  => {
                                [0, 0, 0].into()
                            }
                        };
                    buffer.put_pixel(x, y, color);
                },
                None => {
                    continue;
                }
            }
        }
    }
}
