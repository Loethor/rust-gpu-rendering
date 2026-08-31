//! Named colors used by the examples.
//! Add your own here instead of writing Color::new(...) everywhere.

use crate::math::Color;

pub const BLACK: Color = Color::zero();
pub const WHITE: Color = Color::one();

pub const RED:    Color = Color::new(0.9, 0.2, 0.2);
pub const GREEN:  Color = Color::new(0.2, 0.8, 0.2);
pub const BLUE:   Color = Color::new(0.2, 0.4, 0.9);
pub const ORANGE: Color = Color::new(0.95, 0.45, 0.20);
pub const GRAY:   Color = Color::new(0.5, 0.5, 0.5);

pub const SKY_TOP:     Color = Color::new(0.5, 0.7, 1.0);
pub const SKY_HORIZON: Color = WHITE;

/// Converts a Color with [0,1] floats into RGBA bytes.
/// Applies gamma correction (linear -> sRGB-ish) so dark
/// gradients look smooth instead of crushed and banded.
pub fn to_rgba8(c: Color) -> [u8; 4] {
    let gamma = 1.0 / 2.2;
    [
        (c.x.clamp(0.0, 1.0).powf(gamma) * 255.0) as u8,
        (c.y.clamp(0.0, 1.0).powf(gamma) * 255.0) as u8,
        (c.z.clamp(0.0, 1.0).powf(gamma) * 255.0) as u8,
        255,
    ]
}