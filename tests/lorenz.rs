use jellyfish::dynamics::{DynamicalSystem, Lorenz};
use jellyfish::math::Vec3;

#[test]
fn lorenz_derivative() {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };
    let state = Vec3::new([1.0, 2.0, 3.0]);
    let result = system.derivative(0.0, &state);
    let expected = Vec3::new([10.0, 23.0, -6.0]);

    assert_eq!(result, expected);
}
