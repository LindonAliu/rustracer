// raytracer
// File description:
// framebuffer
//

use std::vec::Vec;
use crate::material::Color;

pub struct Framebuffer {
    pub colors: Vec<Color>,
    pub height: usize,
    pub width: usize
}

impl Framebuffer {

    pub fn new(width: usize, height: usize) -> Framebuffer {
        let mut colors = Vec::new();
        colors.resize(width * height, Color {r:0, g:0, b:0, a:255});
        Framebuffer {
            colors,
            height,
            width
        }
    }

    pub fn get(&self, x: usize, y: usize)-> &Color {
        &self.colors[(self.width * y) + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: Color) {
        self.colors[(self.width * y) + x] = val
    }

}
