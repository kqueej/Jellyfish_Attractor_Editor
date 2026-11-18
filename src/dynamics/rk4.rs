use super::DynamicalSystem;
use crate::math::Vector;

pub fn rk4_step<S, const N: usize>(system: &S, time: f64, state: &Vector<N>, step: f64) -> Vector<N>
where
    S: DynamicalSystem<N>,
{
    let k1 = system.derivative(time, state);
    let midpoint = *state + k1 * (step / 2.0);
    let midtime = time + step / 2.0;
    let k2 = system.derivative(midtime, &midpoint);
    let midpoint2 = *state + k2 * (step / 2.0);
    let k3 = system.derivative(midtime, &midpoint2);
    let endpoint = *state + k3 * step;
    let endtime = time + step;
    let k4 = system.derivative(endtime, &endpoint);
    *state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (step / 6.0)
}
