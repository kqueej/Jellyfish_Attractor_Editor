use crate::dynamics::Lorenz;
use crate::graphics::vertex::Vertex;
use crate::math::Vec3;

use super::attractor::generate_vertices;

pub fn lorenz_vertices() -> Vec<Vertex> {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let initial = Vec3::new([1.0, 1.0, 1.0]);

    generate_vertices(&system, initial, 0.01, 10_000, 1_000)
}
