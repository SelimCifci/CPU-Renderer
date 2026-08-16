#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let r = (color.r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (color.g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (color.b.clamp(0.0, 1.0) * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }
}
