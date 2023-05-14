//
// EPITECH PROJECT, 2023
// polynomial
// File description:
// FreeKOSOVO
//

use crate::vector3d::{Vector3D};

pub fn sq(val: f64) -> f64 {
    val * val
}

pub fn intersect_polynomial(polynomial: Vector3D) -> Option<f64> {
    let delta: f64 = sq(polynomial.y) - (4.0 * polynomial.x * polynomial.z);
    let x1: f64 = (-polynomial.y + delta.sqrt()) / (2. * polynomial.x);
    let x2: f64 = (-polynomial.y - delta.sqrt()) / (2. * polynomial.x);

    if delta < 0. || (x1 < 0. && x2 < 0.) || delta.is_nan() {
        return None;
    }

    if delta == 0. || (x1 >= 0. && x2 < 0.) {
        Some(x1)
    } else if x2 >= 0. && x1 < 0. {
        Some(x2)
    } else {
        Some(x1.min(x2))
    }
}
