// src/render/shading.rs
//! How surfaces respond to light (BRDFs, in renderer jargon).

use crate::math::Color;

/// Ideal matte (Lambertian) surface.
/// `diffuse` is `max(0, dot(normal, light_dir))`, computed by the caller.
pub fn lambert(albedo: Color, diffuse: f32, in_shadow: bool, ambient: f32) -> Color {
    let intensity = if in_shadow {
        ambient
    } else {
        ambient + (1.0 - ambient) * diffuse
    };
    albedo * intensity
}
