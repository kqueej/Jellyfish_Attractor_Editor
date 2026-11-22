use jellyfish::dynamics::{Attractor, DynamicalSystem, Rossler, generate_trajectory, rk4_step};
use jellyfish::math::Vec3;

#[test]
fn rossler_derivative() {
    let system = Rossler {
        a: 0.2,
        b: 0.2,
        c: 5.7,
    };

    let state = Vec3::new([1.0, 2.0, 3.0]);
    let derivative = system.derivative(0.0, &state);

    let [dx, dy, dz] = *derivative.as_array();

    assert!((dx + 5.0).abs() < 1e-12);
    assert!((dy - 1.4).abs() < 1e-12);
    assert!((dz + 13.9).abs() < 1e-12);
}

#[test]
fn rossler_trajectory() {
    let attractor = Attractor::Rossler(Rossler {
        a: 0.2,
        b: 0.2,
        c: 5.7,
    });

    let initial = Vec3::new([1.0, 0.0, 0.0]);

    let trajectory = generate_trajectory(&attractor, initial, 0.01, 10_000, 1_000, rk4_step);

    assert_eq!(trajectory.len(), 10_001);

    assert!(
        trajectory
            .iter()
            .all(|point| { point.as_array().iter().all(|x| x.is_finite()) })
    );
}
