//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// material
//

/// Contains the RGBA values of a color.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Contains the different materials that can be used.
#[derive(Copy, Clone, Debug)]
pub enum Material {
    /// Represents a color, with optional transparency.
    Color(Color),
    /// Represents a mirror.
    Mirror,
}
