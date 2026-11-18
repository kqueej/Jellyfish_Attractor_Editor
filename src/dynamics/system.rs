use crate::math::Vector;

pub trait DynamicalSystem<const N: usize> {
    fn derivative(&self, time: f64, state: &Vector<N>) -> Vector<N>;
}
