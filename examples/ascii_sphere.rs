use rust_gpu_rendering::{project_point, Vec3};

fn main() {
    const WIDTH: usize = 60;
    const HEIGHT: usize = 20;
    const RADIUS: f64 = 1.0;

    let mut frame = vec![vec![' '; WIDTH]; HEIGHT];

    for lat in 0..=180 {
        let theta = (lat as f64) * std::f64::consts::PI / 180.0;
        for lon in 0..=360 {
            let phi = (lon as f64) * std::f64::consts::PI / 180.0;

            let point = Vec3::new(
                RADIUS * theta.sin() * phi.cos(),
                RADIUS * theta.cos(),
                RADIUS * theta.sin() * phi.sin(),
            );

            let (x, y) = project_point(point, 3.0);
            let px = ((x + 1.5) / 3.0 * (WIDTH as f64 - 1.0)) as usize;
            let py = ((1.5 - (y + 1.5) / 3.0) * (HEIGHT as f64 - 1.0)) as usize;

            if px < WIDTH && py < HEIGHT {
                frame[py][px] = '*';
            }
        }
    }

    for row in frame {
        let text: String = row.iter().collect();
        println!("{}", text.trim_end());
    }
}
