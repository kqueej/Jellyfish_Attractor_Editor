use jellyfish::dynamics::{DynamicalSystem, euler_step, generate_trajectory, rk4_step};
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

#[test]
fn rk4_more_accurate_than_euler() {
    let system = Exponential;
    let state = Vector::<1>::new([1.0]);
    let step = 0.1_f64;
    let euler = euler_step(&system, 0.0, &state, step);
    let rk4 = rk4_step(&system, 0.0, &state, step);
    let expected = step.exp();
    let euler_error = (euler.as_array()[0] - expected).abs();
    let rk4_error = (rk4.as_array()[0] - expected).abs();
    assert!(rk4_error < euler_error);
}

#[test]
fn rk4_trajectory_accuracy() {
    let system = Exponential;
    let initial = Vector::<1>::new([1.0]);
    let trajectory = generate_trajectory(&system, initial, 0.1, 10, 0, rk4_step);
    let last = trajectory.last().unwrap();
    let actual = last.as_array()[0];
    let expected = std::f64::consts::E;
    assert!((actual - expected).abs() < 1e-5);
}

#[test]
fn rk4_trajectory_with_warmup() {
    let system = Exponential;
    let initial = Vector::<1>::new([1.0]);
    let trajectory = generate_trajectory(&system, initial, 0.1, 10, 10, rk4_step);

    let first = trajectory[0].as_array()[0];
    let last = trajectory[10].as_array()[0];
    let expected_first = std::f64::consts::E;
    let expected_last = std::f64::consts::E.powi(2);

    assert_eq!(trajectory.len(), 11);
    assert!((first - expected_first).abs() < 2e-5);
    assert!((last - expected_last).abs() < 2e-5);
}
