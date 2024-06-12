//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// trace_rays
//

use crate::intersection::Intersection;
use crate::intersection::Ray;
use crate::light::Light;
use crate::material::Color;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::scene::Scene;
use crate::shape::Shape;
use crate::vector3d::Vector3D;
use nannou::image::DynamicImage;
use nannou::image::RgbImage;

struct InfColor {
    r: f64,
    g: f64,
    b: f64,
}

fn correct_gamma(color: InfColor) -> Color {
    Color {
        r: ((1. - 1. / (2. * color.r / 255. + 1.)) * 255.) as u8,
        g: ((1. - 1. / (2. * color.g / 255. + 1.)) * 255.) as u8,
        b: ((1. - 1. / (2. * color.b / 255. + 1.)) * 255.) as u8,
        a: 255,
    }
}

fn modify_lights(
    color: &Color,
    lights: &Vec<Box<dyn Light>>,
    intersection: &Intersection,
    shape: &dyn Shape,
) -> Color {
    let mut infcolor: InfColor = InfColor {
        r: 0.,
        g: 0.,
        b: 0.,
    };

    for light in lights {
        let color = light.light(intersection, color, shape);
        infcolor.r += color.r as f64;
        infcolor.g += color.g as f64;
        infcolor.b += color.b as f64;
    }
    correct_gamma(infcolor)
}

pub fn reflect(v: Vector3D, n: Vector3D) -> Vector3D {
    v - n.scale(2.0 * v.dot(n))
}

pub fn trace_ray(ray: &Ray, shape: &dyn Shape, lights: &Vec<Box<dyn Light>>) -> Color {
    match shape.intersect(ray) {
        None => Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        Some(result) => match result.material {
            Material::Color(color) => {
                if color.a == 255 {
                    return modify_lights(&color, lights, &result, shape);
                }
                let mut new_color = modify_lights(&color, lights, &result, shape);
                new_color.a = color.a;
                color
            }
            Material::Mirror => {
                let reflected_direction = reflect(ray.direction, result.normal);
                let reflected_ray = Ray {
                    origin: result.intersection_point,
                    direction: reflected_direction,
                };
                trace_ray(&reflected_ray, shape, lights)
            }
        },
    }
}

fn fill_matrix(matrix: &mut Matrix, index: usize, val: Vector3D) {
    matrix[(index, 0)] = val.x;
    matrix[(index, 1)] = val.y;
    matrix[(index, 2)] = val.z;
}

pub fn get_camera_transformation(scene: &Scene) -> Matrix {
    let y: Vector3D = Vector3D {
        x: (0.0),
        y: (1.0),
        z: (0.0),
        w: (0.0),
    };
    let z: Vector3D = scene.camera.direction.normalize();
    let x: Vector3D = y.cross(z).normalize();

    let y: Vector3D = z.cross(x);

    let mut res: Matrix = Matrix::identity(4);

    fill_matrix(&mut res, 0, x);
    fill_matrix(&mut res, 1, y);
    fill_matrix(&mut res, 2, z);
    res
}

pub fn trace_rays(scene: &Scene, framebuffer: &mut DynamicImage) {
    let buffer: &mut RgbImage = framebuffer.as_mut_rgb8().unwrap();
    let width = scene.camera.width as f64;
    let height = scene.camera.height as f64;
    let tan_a = (scene.camera.fov.to_radians() / 2.0).tan();
    let tan_b = tan_a * (width / height);
    let delta = (2.0 * tan_b) / width;
    let transform_cam = get_camera_transformation(scene);

    for x in 0..scene.camera.width {
        for y in 0..scene.camera.height {
            let tmp: Vector3D = Vector3D {
                x: (-tan_b + (x as f64 * delta)),
                y: (-tan_a + (y as f64 * delta)),
                z: (1.0),
                w: (1.0),
            };
            let ray: Ray = Ray {
                origin: (scene.camera.position),
                direction: (transform_cam.clone() * tmp),
            };
            let Color { r, g, b, .. } = trace_ray(&ray, scene.shape.as_ref(), &scene.lights);
            buffer.put_pixel(x, y, [r, g, b].into());
        }
    }
}
