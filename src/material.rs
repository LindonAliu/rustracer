//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// material
//

/// Contains the RGBA values of a color.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Contains the different materials that can be used.
#[derive(Copy, Clone, Debug)]
pub enum Material {
    /// Represents a color, with optional transparency.
    Color(Color),
    /// Represents a mirror.
    Mirror,
}
