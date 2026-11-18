use jellyfish::dynamics::{DynamicalSystem, rk4_step};
use jellyfish::math::Vector;

struct Exponential;

impl DynamicalSystem<1> for Exponential {
    fn derivative(&self, _time: f64, state: &Vector<1>) -> Vector<1> {
        *state
    }
}

#[test]
fn rk4_exponential() {
    let system = Exponential;
    let state = Vector::<1>::new([1.0]);
    let result = rk4_step(&system, 0.0, &state, 0.1);
    let expected = 0.1_f64.exp();
    let actual = result.as_array()[0];
    assert!((actual - expected).abs() < 1e-6);
}
