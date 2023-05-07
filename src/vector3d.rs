//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// vector 3d type
//

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

pub type Point3D = Vector3D;

#[derive(Copy, Clone, Debug)]
pub struct Vector3D {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn length(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: Vector3D) -> f64 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Index<usize> for Vector3D {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.w,
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Vector3D) {
        self.w += other.w;
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, other: Vector3D) {
        self.w -= other.w;
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: f64) -> Vector3D {
        Vector3D {
            w: self.w * other,
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, other: f64) {
        self.w *= other;
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            w: self.w * other.w,
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, other: Vector3D) {
        self.w *= other.w;
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: f64) -> Vector3D {
        Vector3D {
            w: self.w / other,
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, other: f64) {
        self.w /= other;
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Vector3D) -> Vector3D {
        Vector3D {
            w: self.w / other.w,
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign<Vector3D> for Vector3D {
    fn div_assign(&mut self, other: Vector3D) {
        self.w /= other.w;
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
