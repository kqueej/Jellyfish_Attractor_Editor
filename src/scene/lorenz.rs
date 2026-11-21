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

    normalize_xyz(&points)
}
