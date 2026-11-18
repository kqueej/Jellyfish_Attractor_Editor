use super::{DynamicalSystem, euler_step};
use crate::math::Vector;

pub fn generate_trajectory<S, const N: usize>(
    system: &S,
    initial_state: Vector<N>,
    step: f64,
    count: usize,
) -> Vec<Vector<N>>
where
    S: DynamicalSystem<N>,
{
    let mut trajectory = Vec::with_capacity(count + 1);
    let mut state = initial_state;
    let mut time = 0.0;
    trajectory.push(state);

    for _ in 0..count {
        state = euler_step(system, time, &state, step);
        time += step;

        trajectory.push(state);
    }
    trajectory
}
