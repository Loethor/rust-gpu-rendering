// examples/ascii_sphere.rs
//
// Run with: cargo run --example ascii_sphere

use rust_gpu_rendering::math::{Point3, Ray, Vec3};
use rust_gpu_rendering::scene::{Camera, Sphere};
use rust_gpu_rendering::colors as palette;

fn main() {
    let width = 80;
    let height = 40;
    
    // Terminal characters are roughly twice as tall as they are wide.
    let aspect_ratio = (width as f32 / height as f32) / 2.0;

    // Scene setup using our new library types!
    let camera = Camera::new(Point3::zero(), aspect_ratio);
    let sphere = Sphere::diffuse(Point3::new(0.0, 0.0, -1.0), 0.5, palette::WHITE);
    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();

    for y in 0..height {
        for x in 0..width {
            // Map pixel coordinates to [-1, 1] space
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            let v = -((y as f32 / height as f32) * 2.0 - 1.0); 

            // The Camera handles the math of creating the ray now!
            let ray = camera.ray_through(u, v);

            let character = get_character(ray, &sphere, light_dir);
            print!("{character}");
        }
        println!();
    }
}

fn get_character(ray: Ray, sphere: &Sphere, light_dir: Vec3) -> char {
    // The Sphere handles its own intersection math now!
    let Some(t) = sphere.hit(&ray) else {
        return ' '; // Background
    };

    // We hit the sphere! Calculate the exact 3D hit point and surface normal.
    let hit_point = ray.at(t);
    let normal = (hit_point - sphere.center).normalized();
    
    // Lambertian diffuse lighting
    let diffuse = normal.dot(light_dir).max(0.0);
    
    // Add ambient light so the shadow side isn't pitch black
    let ambient = 0.15;
    let shade = ambient + (1.0 - ambient) * diffuse;

    // Map the shade [0.15, 1.0] to an ASCII character ramp
    let ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let index = ((shade * (ramp.len() - 1) as f32).round() as usize).min(ramp.len() - 1);
    
    ramp[index]
}