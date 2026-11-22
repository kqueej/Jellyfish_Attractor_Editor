use super::{DynamicalSystem, Lorenz, Rossler};
use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Attractor {
    Lorenz(Lorenz),
    Rossler(Rossler),
}

impl DynamicalSystem<3> for Attractor {
    fn derivative(&self, time: f64, state: &Vec3) -> Vec3 {
        match self {
            Self::Lorenz(system) => system.derivative(time, state),
            Self::Rossler(system) => system.derivative(time, state),
        }
    }
}
