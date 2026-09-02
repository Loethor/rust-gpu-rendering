use crate::colors as palette;
use crate::math::{Color, Point3, Ray, Vec3};
use crate::scene::{Camera, Material, Sphere};

use super::RenderConfig;

/// Renders the scene and returns one Color per pixel, row-major.
pub fn render(
    camera: &Camera,
    scene: &[Sphere],
    light_dir: Vec3,
    config: &RenderConfig,
    width: u32,
    height: u32,
) -> Vec<Color> {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            let v = -((y as f32 / height as f32) * 2.0 - 1.0);

            let ray = camera.ray_through(u, v);
            pixels.push(shade_pixel(&ray, scene, light_dir, config));
        }
    }

    pixels
}

/// A pixel = average of N rays (N = samples_per_pixel).
/// Without randomness the samples are identical; the knob is wired
/// and becomes anti-aliasing in Phase 3.
fn shade_pixel(ray: &Ray, scene: &[Sphere], light_dir: Vec3, config: &RenderConfig) -> Color {
    let samples = config.samples_per_pixel.max(1);
    let mut acc = Color::zero();

    for _ in 0..samples {
        acc = acc + shade_ray(ray, scene, light_dir, config);
    }

    acc / samples as f32
}

/// Traces one ray through the scene, bouncing up to max_bounces times.
fn shade_ray(ray: &Ray, scene: &[Sphere], light_dir: Vec3, config: &RenderConfig) -> Color {
    let mut ray = *ray;
    let color = Color::zero();
    let mut multiplier = Color::one();

    for _ in 0..config.max_bounces {
        let Some((sphere, hit_point, normal)) = trace_ray(&ray, scene) else {
            // Missed everything -> sky
            return color + multiplier * sky_color(&ray);
        };

        match sphere.material {
            Material::Diffuse { albedo } => {
                let diffuse = normal.dot(light_dir).max(0.0);

                let shadow_ray = Ray::new(hit_point + normal * Sphere::EPSILON, light_dir);
                let in_shadow = config.shadows
                    && diffuse > 0.0
                    && scene
                        .iter()
                        .any(|s| s.hit_after(&shadow_ray, Sphere::EPSILON).is_some());

                let ambient = 0.15;
                let intensity = if in_shadow {
                    ambient
                } else {
                    ambient + (1.0 - ambient) * diffuse
                };

                return color + multiplier * (albedo * intensity);
            }
            Material::Metal { albedo, fuzz: _ } => {
                let dir = ray.direction.reflect(normal);
                ray = Ray::new(hit_point + normal * Sphere::EPSILON, dir);
                multiplier = multiplier * albedo;
            }
        }
    }

    color // bounce budget exhausted: remaining light is lost
}

/// Closest hit in the scene, if any.
fn trace_ray<'a>(ray: &Ray, scene: &'a [Sphere]) -> Option<(&'a Sphere, Point3, Vec3)> {
    let mut closest_t = f32::INFINITY;
    let mut hit_data = None;

    for sphere in scene {
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
