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
        for y in 0..scene.camera.height {
            ray.direction.x = -tan_b + (x as f64 * delta);
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
