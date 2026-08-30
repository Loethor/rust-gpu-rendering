use rust_gpu_rendering::math::{Vec3, Point3, Ray};

fn main() {
    let width = 80;
    let height = 40;
    
    // Terminal characters are roughly twice as tall as they are wide.
    // This corrects the aspect ratio so the sphere doesn't look horizontally squashed.
    let aspect_ratio = (width as f32 / height as f32) / 2.0;

    // Scene setup
    let light_dir = Vec3::new(-0.5, 0.7, 0.6).normalized();
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;

    for y in 0..height {
        for x in 0..width {
            // Map pixel coordinates to [-1, 1] space
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            // Flip y so +y is up
            let v = -((y as f32 / height as f32) * 2.0 - 1.0); 

            // Create a target point on the virtual image plane at z = -1.0
            let target = Point3::new(u * aspect_ratio, v, -1.0);
            let origin = Point3::zero();
            
            // The ray direction from the camera origin to the target pixel
            let direction = (target - origin).normalized();
            let ray = Ray::new(origin, direction);

            let character = get_character(ray, sphere_center, sphere_radius, light_dir);
            print!("{character}");
        }
        println!();
    }
}

fn get_character(r: Ray, center: Point3, radius: f32, light_dir: Vec3) -> char {
    // Ray-sphere intersection math:
    // We are solving: dot(P(t) - center, P(t) - center) = radius^2
    // Which expands to the quadratic equation: a*t^2 + 2*half_b*t + c = 0
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let half_b = oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    
    let discriminant = half_b * half_b - a * c;

    // If discriminant is negative, the ray misses the sphere entirely
    if discriminant < 0.0 {
        return ' '; // Background
    }

    // Find the nearest root (the front face of the sphere)
    let mut t = (-half_b - discriminant.sqrt()) / a;
    if t < 0.0 {
        // If the nearest root is behind the camera, try the far root (inside the sphere looking out)
        t = (-half_b + discriminant.sqrt()) / a;
    }

    // If t is still negative, the sphere is completely behind the camera
    if t < 0.0 {
        return ' '; 
    }

    // We hit the sphere! Calculate the exact 3D hit point and surface normal.
    let hit_point = r.at(t);
    let normal = (hit_point - center).normalized();
    
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