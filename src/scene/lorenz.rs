use crate::dynamics::{Lorenz, generate_trajectory, rk4_step};
use crate::graphics::projection::normalize_xyz;
use crate::graphics::vertex::Vertex;
use crate::math::Vec3;

pub fn lorenz_vertices() -> Vec<Vertex> {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let initial = Vec3::new([1.0, 1.0, 1.0]);

    let trajectory = generate_trajectory(&system, initial, 0.01, 10_000, 1_000, rk4_step);

    let points: Vec<[f64; 3]> = trajectory.iter().map(|point| *point.as_array()).collect();

    let mut vertices = normalize_xyz(&points);

    let speeds: Vec<f64> = trajectory
        .iter()
        .map(|point| lorenz_speed(*point.as_array(), system.sigma, system.rho, system.beta))
        .collect();

    let min_speed = speeds.iter().copied().fold(f64::INFINITY, f64::min);

    let max_speed = speeds.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let speed_range = max_speed - min_speed;

    for (vertex, speed) in vertices.iter_mut().zip(speeds.iter()) {
        vertex.speed = if speed_range > 0.0 {
            ((speed - min_speed) / speed_range) as f32
        } else {
            0.0
        };
    }

    vertices
}

fn lorenz_speed(point: [f64; 3], sigma: f64, rho: f64, beta: f64) -> f64 {
    let [x, y, z] = point;

    let dx = sigma * (y - x);
    let dy = x * (rho - z) - y;
    let dz = x * y - beta * z;

    dx.hypot(dy).hypot(dz)
}
