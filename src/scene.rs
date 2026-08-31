use crate::math::{Point3, Ray, Vec3};

// ==========================================
// Sphere (pure geometry — no appearance)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Distance `t` to the closest hit in front of the camera, if any.
    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-half_b - sqrt_d) / a;
        if t < 0.0 {
            t = (-half_b + sqrt_d) / a; // camera inside the sphere
        }

        if t < 0.0 { None } else { Some(t) }
    }
}

// ==========================================
// Camera
// ==========================================

/// A simple pinhole camera at `origin`, looking toward -z,
/// with the image plane 1 unit in front.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Point3,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(origin: Point3, aspect_ratio: f32) -> Self {
        Self { origin, aspect_ratio }
    }

    /// Ray from the camera through image-plane coordinate (u, v),
    /// where u and v are roughly in [-1, 1].
    pub fn ray_through(&self, u: f32, v: f32) -> Ray {
        let offset = Vec3::new(u * self.aspect_ratio, v, -1.0);
        Ray::new(self.origin, offset.normalized())
    }
}

// ==========================================
// Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hits_sphere_in_front() {
        let s = Sphere::new(Point3::new(0.0, 0.0, -3.0), 1.0);
        let r = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let t = s.hit(&r).unwrap();
        assert!((t - 2.0).abs() < 1e-5);
    }

    #[test]
    fn ray_misses_sphere() {
        let s = Sphere::new(Point3::new(5.0, 0.0, -3.0), 1.0);
        let r = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(s.hit(&r), None);
    }

    #[test]
    fn sphere_behind_camera_is_ignored() {
        let s = Sphere::new(Point3::new(0.0, 0.0, 3.0), 1.0);
        let r = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(s.hit(&r), None);
    }

    #[test]
    fn camera_ray_goes_through_image_plane() {
        let cam = Camera::new(Point3::zero(), 1.0);
        let r = cam.ray_through(0.0, 0.0);
        // Straight ahead: direction (0, 0, -1)
        assert!((r.direction - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-6);
    }
}