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

use vector3d::Vector3D;

fn main() {
    let mut vector3d = Vector3D {x: 3.0, y: 4.0, z: 5.0};
    let second_vector3d = Vector3D {x: 1.0, y: 1.0, z: 1.0};
    vector3d += second_vector3d;
    vector3d -= second_vector3d;

    println!("{:?}", vector3d);
    println!("{:?}", vector3d.length());
}
