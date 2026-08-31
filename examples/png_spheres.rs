// examples/png_spheres.rs
//
// Run with: cargo run --example png_spheres

use image::{Rgba, RgbaImage};
use rust_gpu_rendering::colors as palette;
use rust_gpu_rendering::math::{Color, Point3, Ray, Vec3};
use rust_gpu_rendering::scene::{Camera, Sphere};

fn main() {
    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;

    let camera = Camera::new(Point3::zero(), aspect_ratio);
    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();

    let spheres = [
        // Giant sphere far below = ground
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, palette::GRAY),
        Sphere::new(Point3::new(-0.8, -0.2, -1.5), 0.45, palette::RED),
        Sphere::new(Point3::new( 0.0,  0.3, -1.3), 0.50, palette::GREEN),
        Sphere::new(Point3::new( 0.8, -0.3, -1.1), 0.35, palette::BLUE),
    ];

    let mut img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Map to [-1, 1] space
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            let v = -((y as f32 / height as f32) * 2.0 - 1.0);

            let ray = camera.ray_through(u, v);
            let color = shade_pixel(&ray, &spheres, light_dir);

            let [r, g, b, a] = palette::to_rgba8(color);
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img.save("output.png").expect("Failed to save output.png");
    println!("Saved output.png");
}

fn shade_pixel(ray: &Ray, spheres: &[Sphere], light_dir: Vec3) -> Color {
    // 1. Find the closest hit
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

    // 2. If we hit a sphere, shade it
    if let Some(sphere) = hit_sphere {
        let hit_point = ray.at(closest_t);
        let normal = (hit_point - sphere.center).normalized();

        // Lambertian diffuse lighting
        let diffuse = normal.dot(light_dir).max(0.0);

        // Shadow ray: from the hit point toward the light.
        // If ANY sphere blocks it, the point is in shadow.
        let shadow_ray = Ray::new(hit_point, light_dir);
        let in_shadow = diffuse > 0.0
            && spheres
                .iter()
                .any(|s| s.hit_after(&shadow_ray, Sphere::EPSILON).is_some());

        let ambient = 0.15;
        let light_intensity = if in_shadow {
            ambient // in shadow: ambient only
        } else {
            ambient + (1.0 - ambient) * diffuse
        };

        return sphere.albedo * light_intensity;
    }

    // 3. If we missed everything, draw the sky
    sky_color(ray)
}

/// Smooth gradient based on the ray's Y direction.
fn sky_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * palette::SKY_HORIZON + t * palette::SKY_TOP
}