use jellyfish::dynamics::{Lorenz, generate_trajectory, rk4_step};
use jellyfish::math::Vec3;

#[test]
fn trajectory_generation() {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let initial = Vec3::new([1.0, 2.0, 3.0]);
    let trajectory = generate_trajectory(&system, initial, 0.01, 10, 0, rk4_step);
    assert_eq!(trajectory.len(), 11);
    assert_eq!(trajectory[0], initial);
}

#[test]
fn lorenz_trajectory_is_finite() {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let initial = Vec3::new([1.0, 1.0, 1.0]);

    let trajectory = generate_trajectory(&system, initial, 0.01, 10_000, 1_000, rk4_step);

    assert_eq!(trajectory.len(), 10_001);

    for point in &trajectory {
        for coordinate in point.as_array() {
            assert!(coordinate.is_finite());
        }
    }
}
