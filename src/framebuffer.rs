use rayon::prelude::*;

use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    color_buffer: Vec<u32>,
    z_buffer: Vec<f32>,
    clear_color: Color,
}
impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![0; width * height],
            z_buffer: vec![1.0; width * height],
            clear_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        }
    }
    pub fn get_buffer(&self) -> &[u32] {
        &self.color_buffer
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }
    pub fn clear(&mut self) {
        self.color_buffer
            .par_chunks_mut(self.width)
            .for_each(|row| row.fill(self.clear_color.into()));
    }
    pub fn clear_depth(&mut self) {
        self.z_buffer
            .par_chunks_mut(self.width)
            .for_each(|row| row.fill(1.0));
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        let idx = y as usize * self.width + x as usize;
        self.color_buffer[idx] = color;
    }
    pub fn set_pixel_z(&mut self, x: u32, y: u32, z: f32) {
        let idx = y as usize * self.width + x as usize;
        self.z_buffer[idx] = z;
    }
    pub fn get_pixel_z(&self, x: u32, y: u32) -> f32 {
        let idx = y as usize * self.width + x as usize;
        self.z_buffer[idx]
    }
    pub fn test_z_set_pixel(&mut self, x: u32, y: u32, z: f32, color: u32) -> bool {
        if x as usize >= self.width || y as usize >= self.height {
            return false;
        }

        let idx = y as usize * self.width + x as usize;

        if z < self.z_buffer[idx] {
            self.z_buffer[idx] = z;
            self.color_buffer[idx] = color;
            true
        } else {
            false
        }
    }
}
