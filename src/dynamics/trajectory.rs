use super::DynamicalSystem;
use crate::math::Vector;

pub fn generate_trajectory<S, const N: usize>(
    system: &S,
    initial_state: Vector<N>,
    step: f64,
    count: usize,
    wapmup: usize,
    integrator: fn(&S, f64, &Vector<N>, f64) -> Vector<N>,
) -> Vec<Vector<N>>
where
    S: DynamicalSystem<N>,
{
    let mut trajectory = Vec::with_capacity(count + 1);
    let mut state = initial_state;
    let mut time = 0.0;

    for _ in 0..wapmup {
        state = integrator(system, time, &state, step);
        time += step;
    }

    trajectory.push(state);

    for _ in 0..count {
        state = integrator(system, time, &state, step);
        time += step;

        trajectory.push(state);
    }

    trajectory
}
