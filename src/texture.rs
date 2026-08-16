use glam::Vec2;
use image::open;

use crate::color::Color;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Texture {
    pub fn new(width: usize, height: usize, data: Vec<Color>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn checkerboard(width: usize, height: usize, grid_size: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let is_white = ((x / grid_size) + (y / grid_size)) % 2 == 0;
                if is_white {
                    data.push(Color::BLACK);
                } else {
                    data.push(Color::new(1.0, 0.0, 1.0));
                }
            }
        }
        Self::new(width, height, data)
    }

    pub fn load_from_file(path: &str) -> Self {
        let img = open(path).expect("Failed to load texture").to_rgba8();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let data = img
            .pixels()
            .map(|p| {
                Color::new(
                    p[0] as f32 / 255.0,
                    p[1] as f32 / 255.0,
                    p[2] as f32 / 255.0,
                )
            })
            .collect();
        Self::new(width, height, data)
    }

    pub fn sample(&self, uv: Vec2) -> Color {
        let u = uv.x.rem_euclid(1.0);
        let v = uv.y.rem_euclid(1.0);

        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = (((1.0 - v) * self.height as f32) as usize).min(self.height - 1);

        let idx = y * self.width + x;
        self.data[idx]
    }
}
