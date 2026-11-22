use crate::dynamics::{DynamicalSystem, generate_trajectory, rk4_step};
use crate::graphics::projection::normalize_xyz;
use crate::graphics::vertex::Vertex;
use crate::math::Vec3;

pub fn generate_vertices<S>(
    system: &S,
    initial: Vec3,
    step: f64,
    count: usize,
    warmup: usize,
) -> Vec<Vertex>
where
    S: DynamicalSystem<3>,
{
    let trajectory = generate_trajectory(system, initial, step, count, warmup, rk4_step);

    let points: Vec<[f64; 3]> = trajectory.iter().map(|point| *point.as_array()).collect();

    let mut vertices = normalize_xyz(&points);

    let speeds: Vec<f64> = trajectory
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let time = (warmup + index) as f64 * step;

            let derivative = system.derivative(time, point);
            let [dx, dy, dz] = *derivative.as_array();

            dx.hypot(dy).hypot(dz)
        })
        .collect();

    let min_speed = speeds.iter().copied().fold(f64::INFINITY, f64::min);

    let max_speed = speeds.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let speed_range = max_speed - min_speed;

    for (vertex, speed) in vertices.iter_mut().zip(speeds.iter()) {
        vertex.speed = if speed_range > 0.0 && speed_range.is_finite() {
            ((speed - min_speed) / speed_range) as f32
        } else {
            0.0
        };
    }

    vertices
}
