// src/render/framebuffer.rs
use crate::colors;
use crate::math::Color;

/// A rendered image: linear-light colors, row-major.
#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: Vec::with_capacity((width * height) as usize),
        }
    }

    /// Gamma-corrected RGBA8 bytes, ready for any image encoder.
    pub fn to_rgba8_bytes(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|c| colors::to_rgba8(*c))
            .collect()
    }
}

#[cfg(feature = "png")]
impl Framebuffer {
    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        image::save_buffer(
            path,
            &self.to_rgba8_bytes(),
            self.width,
            self.height,
            image::ColorType::Rgba8,
        )
    }
}
