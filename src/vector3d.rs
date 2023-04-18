//
// EPITECH PROJECT, 2023
// vector
// File description:
// FreeKOSOVO
//

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Vector3D) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, other: Vector3D) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: f64) -> Vector3D {
        Vector3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, other: Vector3D) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: f64) -> Vector3D {
        Vector3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign<Vector3D> for Vector3D {
    fn div_assign(&mut self, other: Vector3D) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
