// examples/png_spheres.rs
//
// Run with: cargo run --example png_spheres

use image::{Rgba, RgbaImage};
use rust_gpu_rendering::colors as palette;
use rust_gpu_rendering::math::{Color, Point3, Ray, Vec3};
use rust_gpu_rendering::scene::{Camera, Material, Sphere};

fn main() {
    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;

    let camera = Camera::new(Point3::zero(), aspect_ratio);
    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();

    let spheres = [
        // Ground 
        Sphere::diffuse(Point3::new(0.0, -100.5, -1.0), 100.0, palette::GRAY),
        // Left: Matte Red
        Sphere::diffuse(Point3::new(-0.8, -0.2, -1.5), 0.45, palette::RED),
        // Center: Shiny Green Metal
        Sphere::metal(Point3::new(0.0, 0.3, -1.3), 0.50, palette::GREEN, 0.0),
        // Right: Shiny Blue Metal
        Sphere::metal(Point3::new(0.8, -0.3, -1.1), 0.35, palette::BLUE, 0.0),
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

fn shade_pixel(initial_ray: &Ray, spheres: &[Sphere], light_dir: Vec3) -> Color {
    let mut ray = *initial_ray;
    let mut final_color = Color::zero();
    
    // Multiplier tracks the "energy" the ray carries as it bounces.
    // Starts at pure white (1.0, 1.0, 1.0).
    let mut multiplier = Color::one(); 

    // Limit bounces to prevent infinite loops (e.g., two mirrors facing each other)
    for _bounce in 0..3 { 
        let hit = trace_ray(&ray, spheres);

        let Some((sphere, hit_point, normal)) = hit else {
            // Missed everything -> Sky
            final_color = final_color + multiplier * sky_color(&ray);
            break;
        };

        match sphere.material {
            Material::Diffuse { albedo } => {
                // Diffuse objects absorb the ray after calculating direct light & shadows
                let diffuse = normal.dot(light_dir).max(0.0);
                
                // Nudge the shadow ray origin slightly along the normal to prevent self-intersection
                let shadow_ray = Ray::new(hit_point + normal * Sphere::EPSILON, light_dir);
                let in_shadow = diffuse > 0.0
                    && spheres.iter().any(|s| s.hit_after(&shadow_ray, Sphere::EPSILON).is_some());

                let ambient = 0.15;
                let light_intensity = if in_shadow { ambient } else { ambient + (1.0 - ambient) * diffuse };

                final_color = final_color + multiplier * (albedo * light_intensity);
                break; // Ray stops here
            }
            Material::Metal { albedo, fuzz: _ } => {
                // Metal objects bounce the ray!
                // We nudge the origin by EPSILON along the normal to prevent self-intersection
                let bounce_origin = hit_point + normal * Sphere::EPSILON;
                let bounce_dir = ray.direction.reflect(normal);
                
                ray = Ray::new(bounce_origin, bounce_dir);
                
                // The metal tints the reflected light (e.g., gold tints reflections yellow)
                multiplier = multiplier * albedo;
                // The loop continues to the next bounce!
            }
        }
    }
    final_color
}

/// Helper to find the closest hit in the scene.
/// Notice we use `hit_after(ray, EPSILON)` for ALL rays now (primary, shadow, and bounce).
fn trace_ray<'a>(ray: &Ray, spheres: &'a [Sphere]) -> Option<(&'a Sphere, Point3, Vec3)> {
    let mut closest_t = f32::INFINITY;
    let mut hit_data = None;

    for sphere in spheres {
        if let Some(t) = sphere.hit_after(ray, Sphere::EPSILON) {
            if t < closest_t {
                closest_t = t;
                let point = ray.at(t);
                let normal = (point - sphere.center).normalized();
                hit_data = Some((sphere, point, normal));
            }
        }
    }
    hit_data
}

fn sky_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * palette::SKY_HORIZON + t * palette::SKY_TOP
}