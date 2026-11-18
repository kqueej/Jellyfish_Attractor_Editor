use super::DynamicalSystem;
use crate::math::Vector;

pub fn euler_step<S, const N: usize>(
    system: &S,
    time: f64,
    state: &Vector<N>,
    step: f64,
) -> Vector<N>
where
    S: DynamicalSystem<N>,
{
    let derivative = system.derivative(time, state);
    *state + derivative * step
}
