use jellyfish::dynamics::{Lorenz, euler_step};
use jellyfish::math::Vec3;

#[test]
fn euler_lorenz_step() {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let state = Vec3::new([1.0, 2.0, 3.0]);

    let result = euler_step(&system, 0.0, &state, 0.01);
    let expected = Vec3::new([1.1, 2.23, 2.94]);

    for i in 0..3 {
        let actual = result.as_array()[i];
        let target = expected.as_array()[i];

        assert!(
            (actual - target).abs() < 1e-12,
            "Coordinate {i}: expected {target}, got {actual}"
        );
    }
}
