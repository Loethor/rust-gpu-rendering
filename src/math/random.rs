// src/math/random.rs

use super::Vec3;
use rand::Rng;

/// Returns a random 3D vector inside the unit sphere (radius 1).
/// Uses rejection sampling: pick a point in a [-1, 1] cube,
/// throw it away if it falls outside the sphere.
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Returns a random 3D vector exactly on the surface of the unit sphere.
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalized()
}

/// Returns a random 3D vector in the same hemisphere as the given normal.
/// Crucial for bouncing light *away* from a surface, not back into it.
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere // flip it to the same side as the normal
    }
}

/// Returns a random 2D point (with z=0) inside the unit disk.
/// We won't use this until much later (Depth of Field), but it's good to have ready!
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

// ==========================================
// Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_points_are_inside_unit_sphere() {
        for _ in 0..1000 {
            let v = random_in_unit_sphere();
            assert!(v.length_squared() < 1.0);
        }
    }

    #[test]
    fn random_unit_vectors_have_length_one() {
        for _ in 0..1000 {
            let v = random_unit_vector();
            // Check that length is 1.0 (with a tiny float tolerance)
            assert!((v.length() - 1.0).abs() < 1e-5);
        }
    }
}
