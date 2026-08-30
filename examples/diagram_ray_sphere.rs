// examples/diagram_ray_sphere.rs
//
// Regenerate with: cargo run --example diagram_ray_sphere

use plotters::coord::Shift;
use plotters::prelude::*;

const SCALE: f64 = 120.0;
const ORIGIN: (f64, f64) = (600.0, 430.0);

// Concrete drawing area type.
// BitMapBackend borrows its pixel buffer, so it carries a lifetime parameter.
type Area<'a> = DrawingArea<BitMapBackend<'a>, Shift>;

fn px(p: (f64, f64)) -> (i32, i32) {
    (
        (ORIGIN.0 + p.0 * SCALE) as i32,
        (ORIGIN.1 - p.1 * SCALE) as i32, // flip y so +y is up
    )
}

fn arrow(
    root: &Area<'_>,
    from: (f64, f64),
    to: (f64, f64),
    color: RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    let style = color.stroke_width(4);
    root.draw(&PathElement::new(vec![px(from), px(to)], style))?;

    // Arrow head: two short segments rotated ±25° from the backward direction
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let len = (dx * dx + dy * dy).sqrt();
    let (ux, uy) = (dx / len, dy / len);
    let a = 25f64.to_radians();
    let (ca, sa) = (a.cos(), a.sin());
    let head = 0.22;
    let v = (-ux, -uy);
    let r1 = (v.0 * ca - v.1 * sa, v.0 * sa + v.1 * ca);
    let r2 = (v.0 * ca + v.1 * sa, -v.0 * sa + v.1 * ca);
    let h1 = (to.0 + head * r1.0, to.1 + head * r1.1);
    let h2 = (to.0 + head * r2.0, to.1 + head * r2.1);

    root.draw(&PathElement::new(vec![px(h1), px(to), px(h2)], style))?;
    Ok(())
}

fn label(
    root: &Area<'_>,
    text: &str,
    at: (f64, f64),
    color: RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    root.draw(&Text::new(
        text,
        px(at),
        ("sans-serif", 26).into_font().color(&color),
    ))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("images")?;

    let root =
        BitMapBackend::new("images/ray-sphere-intersection.png", (1200, 800))
            .into_drawing_area();
    root.fill(&WHITE)?;

    // ---- Scene (same names as ascii_sphere.rs) ----
    let center: (f64, f64) = (0.7, 0.2);
    let radius = 1.7;
    let r_origin: (f64, f64) = (-2.6, -2.2);

    // Hit point P(t): a point on the circle at 50°
    let ang = 50f64.to_radians();
    let p_t = (
        center.0 + radius * ang.cos(),
        center.1 + radius * ang.sin(),
    );

    // ---- Sphere ----
    root.draw(&Circle::new(
        px(center),
        (radius * SCALE) as i32,
        BLACK.stroke_width(4),
    ))?;

    // Radius line
    root.draw(&PathElement::new(
        vec![px(center), px((center.0 + radius, center.1))],
        BLACK.stroke_width(2),
    ))?;
    label(&root, "radius", (center.0 + radius * 0.45, center.1 - 0.22), BLACK)?;

    // ---- Vectors ----
    arrow(&root, center, r_origin, RED)?;   // oc = r.origin - center
    arrow(&root, center, p_t, GREEN)?;      // P(t) - center (length == radius)
    arrow(&root, r_origin, p_t, BLUE)?;     // the ray: origin + t * direction

    // ---- Points ----
    root.draw(&Circle::new(px(center), 7, BLACK.filled()))?;
    root.draw(&Circle::new(px(r_origin), 7, BLACK.filled()))?;
    root.draw(&Circle::new(px(p_t), 7, BLACK.filled()))?;

    // ---- Labels (tweak positions to taste) ----
    label(&root, "center", (0.75, 0.0), BLACK)?;
    label(&root, "r.origin", (r_origin.0 - 0.4, r_origin.1 - 0.4), BLACK)?;
    label(&root, "P(t)", (p_t.0 + 0.12, p_t.1 + 0.1), BLACK)?;
    label(&root, "oc", (-0.6, -1.3), RED)?;
    label(&root, "t", (-1.75, -1.8), BLACK)?;
    label(&root, "r.direction", (-0.9, -0.1), BLUE)?;
    label(&root, "P(t) - center", (1.3, 0.7), GREEN)?;

    // ---- Equations box ----
    root.draw(&Rectangle::new([(770, 30), (1180, 220)], WHITE.filled()))?;
    root.draw(&Rectangle::new([(770, 30), (1180, 220)], BLACK.stroke_width(2)))?;

    let eqs = [
        "oc = r.origin - center",
        "a = r.direction.dot(r.direction)",
        "half_b = oc.dot(r.direction)",
        "c = oc.dot(oc) - radius^2",
        "a*t^2 + 2*half_b*t + c = 0",
    ];
    for (i, line) in eqs.iter().enumerate() {
        root.draw(&Text::new(
            *line,
            (785, 48 + i as i32 * 34),
            ("monospace", 20).into_font().color(&BLACK),
        ))?;
    }

    root.present()?;
    println!("Saved images/ray-sphere-intersection.png");
    Ok(())
}