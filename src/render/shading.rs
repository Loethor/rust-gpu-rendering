//! How surfaces respond to light (BRDFs, in renderer jargon).

use crate::math::{random, Color, Vec3};

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

/// Mirror reflection, optionally roughened by `fuzz`.
/// 0.0 = perfect mirror, 1.0 = very rough metal.
pub fn reflect_metal(direction: Vec3, normal: Vec3, fuzz: f32) -> Vec3 {
    let reflected = direction.reflect(normal);
    if fuzz <= 0.0 {
        return reflected;
    }

    // Perturb the perfect reflection with a random vector,
    // simulating microscopic surface roughness.
    let fuzzed = (reflected + fuzz * random::random_in_unit_sphere()).normalized();

    // At high fuzz, grazing rays can get perturbed *below* the surface.
    // Fall back to the perfect reflection instead of shooting into the object.
    if fuzzed.dot(normal) > 0.0 {
        fuzzed
    } else {
        reflected
    }
}

// ==========================================
// Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_fuzz_is_perfect_mirror() {
        let d = Vec3::new(1.0, -1.0, 0.0).normalized();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let r = reflect_metal(d, n, 0.0);
        let expected = Vec3::new(1.0, 1.0, 0.0).normalized();
        assert!((r - expected).length() < 1e-6);
    }

    #[test]
    fn fuzzed_rays_stay_above_surface() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let d = Vec3::new(0.5, -1.0, 0.2).normalized();
        for _ in 0..1000 {
            let r = reflect_metal(d, n, 1.0);
            // Never into the surface, always a unit vector
            assert!(r.dot(n) > 0.0);
            assert!((r.length() - 1.0).abs() < 1e-5);
        }
    }
}
