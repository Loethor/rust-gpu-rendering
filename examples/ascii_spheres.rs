// examples/ascii_spheres.rs
//
// Run with: cargo run --example ascii_spheres

use rust_gpu_rendering::math::{Point3, Ray, Vec3};
use rust_gpu_rendering::scene::{Camera, Sphere};
use rust_gpu_rendering::colors as palette;


fn main() {
    let width = 80;
    let height = 40;
    let aspect_ratio = (width as f32 / height as f32) / 2.0;

    let camera = Camera::new(Point3::zero(), aspect_ratio);
    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();

    // The scene: just an array of pure geometry
    let spheres = [
        Sphere::new(Point3::new(-0.7, -0.25, -1.5), 0.45, palette::WHITE),
        Sphere::new(Point3::new( 0.0,  0.25, -1.3), 0.40, palette::WHITE),
        Sphere::new(Point3::new( 0.7, -0.30, -1.1), 0.30, palette::WHITE),
    ];

    for y in 0..height {
        for x in 0..width {
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            let v = -((y as f32 / height as f32) * 2.0 - 1.0);

            let ray = camera.ray_through(u, v);
            print!("{}", shade_pixel(ray, &spheres, light_dir));
        }
        println!();
    }
}

/// Shoots a ray into the scene and returns the ASCII character for this pixel.
fn shade_pixel(ray: Ray, spheres: &[Sphere], light_dir: Vec3) -> char {
    // Find the CLOSEST hit among all spheres
    let mut closest_t = f32::INFINITY;
    let mut hit_sphere: Option<&Sphere> = None;

    for sphere in spheres {
        if let Some(t) = sphere.hit(&ray) {
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

    // Shade the hit with the sphere's own normal
    let hit_point = ray.at(closest_t);
    let normal = (hit_point - sphere.center).normalized();

    let diffuse = normal.dot(light_dir).max(0.0);
    let ambient = 0.15;
    let shade = ambient + (1.0 - ambient) * diffuse;

    // All spheres share the same material/ramp now
    let ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let index = (shade * (ramp.len() - 1) as f32).round() as usize;
    ramp[index.min(ramp.len() - 1)]
}