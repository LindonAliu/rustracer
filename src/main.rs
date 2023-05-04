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
use intersections::plane::Plane;

use vector3d::Vector3D;
use vector3d::Point3D;
use crate::shape::Shape;
use crate::intersection::Ray;
use material::Material;
use material::Color;

fn main() {
    let mut vector3d = Vector3D {x: 3.0, y: 4.0, z: 5.0};
    let second_vector3d = Vector3D {x: 1.0, y: 1.0, z: 1.0};
    vector3d += second_vector3d;
    vector3d -= second_vector3d;

    println!("{:?}", vector3d);
    println!("{:?}", vector3d.length());
    let plan = Plane {
        pos: Point3D {x: 1.0, y:2.0, z: 3.0},
        normal: Vector3D {x: 6.0, y: 9.0, z: 12.0},
        material: Material::Color(Color {r: 10, g: 200, b: 250, a: 255})
    };

    let ray = Ray {
        origin: Point3D {x: 1.0, y:2.0, z: 3.0},
        direction: Vector3D {x: 6.0, y: 9.0, z: 12.0},
    };

    plan.intersect(&ray);
}
