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
    pub fn get(&self, x: usize, y: usize)-> &Color {
        &self.colors[(self.width * y) + x]
    }
    pub fn set(&mut self, x: usize, y: usize, val: Color) {
        self.colors[(self.width * y) + x] = val
    }
}
