// examples/ascii_spheres.rs
//
// Run with: cargo run --example ascii_spheres

use rust_gpu_rendering::math::{Point3, Ray, Vec3};

// ==========================================
// Sphere
// ==========================================

struct Sphere {
    center: Point3,
    radius: f32,
    /// ASCII shading ramp — acts like a simple "material"
    ramp: &'static [char],
}

impl Sphere {
    /// Returns the distance `t` to the closest hit in front of the camera, if any.
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None; // ray misses this sphere
        }

        let sqrt_d = discriminant.sqrt();

        // Closest root first (front face)
        let mut t = (-half_b - sqrt_d) / a;
        if t < 0.0 {
            // Camera might be inside the sphere: try the far root
            t = (-half_b + sqrt_d) / a;
        }

        if t < 0.0 { None } else { Some(t) }
    }
}

// Different "materials" for different spheres
const RAMP_SMOOTH: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const RAMP_DOTS: &[char] = &[' ', '.', 'o', 'O', '0', '@'];
const RAMP_LINES: &[char] = &[' ', '_', '-', '=', '#', '@'];

// ==========================================
// Renderer
// ==========================================

fn main() {
    let width = 80;
    let height = 40;

    // Terminal characters are ~2x taller than wide
    let aspect_ratio = (width as f32 / height as f32) / 2.0;

    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();

    // The scene: add as many spheres as you want
    let spheres = [
        Sphere { center: Point3::new(-0.7, -0.25, -1.5), radius: 0.45, ramp: RAMP_SMOOTH },
        Sphere { center: Point3::new(0.0, 0.25, -1.3), radius: 0.40, ramp: RAMP_DOTS },
        Sphere { center: Point3::new(0.7, -0.30, -1.1), radius: 0.30, ramp: RAMP_LINES },
    ];

    for y in 0..height {
        for x in 0..width {
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            let v = -((y as f32 / height as f32) * 2.0 - 1.0);

            let target = Point3::new(u * aspect_ratio, v, -1.0);
            let origin = Point3::zero();
            let direction = (target - origin).normalized();
            let ray = Ray::new(origin, direction);

            print!("{}", shade_pixel(&ray, &spheres, light_dir));
        }
        println!();
    }
}

/// Shoots a ray into the scene and returns the ASCII character for this pixel.
fn shade_pixel(ray: &Ray, spheres: &[Sphere], light_dir: Vec3) -> char {
    // Find the CLOSEST hit among all spheres
    let mut closest_t = f32::INFINITY;
    let mut hit_sphere: Option<&Sphere> = None;

    for sphere in spheres {
        if let Some(t) = sphere.hit(ray) {
            if t < closest_t {
                closest_t = t;
                hit_sphere = Some(sphere);
            }
        }
    }

    // No hit -> background
    let Some(sphere) = hit_sphere else {
        return ' ';
    };

    // Shade the hit with the sphere's own normal and "material"
    let hit_point = ray.at(closest_t);
    let normal = (hit_point - sphere.center).normalized();

    let diffuse = normal.dot(light_dir).max(0.0);
    let ambient = 0.15;
    let shade = ambient + (1.0 - ambient) * diffuse;

    let ramp = sphere.ramp;
    let index = (shade * (ramp.len() - 1) as f32).round() as usize;
    ramp[index.min(ramp.len() - 1)]
}