// examples/png_spheres.rs
//
// Run with: cargo run --example png_spheres

use image::{Rgba, RgbaImage};
use rust_gpu_rendering::colors as palette;
use rust_gpu_rendering::math::{Point3, Vec3};
use rust_gpu_rendering::render::{render, RenderConfig};
use rust_gpu_rendering::scene::{Camera, Sphere};

fn main() {
    let width = 800;
    let height = 600;

    let camera = Camera::new(Point3::zero(), width as f32 / height as f32);
    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();

    let scene = [
        Sphere::diffuse(Point3::new(0.0, -100.5, -1.0), 100.0, palette::GRAY),
        Sphere::diffuse(Point3::new(-0.8, -0.2, -1.5), 0.45, palette::RED),
        Sphere::metal(Point3::new(0.0, 0.3, -1.3), 0.50, palette::GREEN, 0.0),
        Sphere::metal(Point3::new(0.8, -0.3, -1.1), 0.35, palette::BLUE, 0.0),
    ];

    // ============ THE CONTROL PANEL ============
    let config = RenderConfig::default().shadows(true).bounces(4).samples(1);
    // ===========================================

    let pixels = render(&camera, &scene, light_dir, &config, width, height);

    let mut img = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let [r, g, b, a] = palette::to_rgba8(pixels[(y * width + x) as usize]);
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img.save("output.png").expect("Failed to save output.png");
    println!("Saved output.png");
}
