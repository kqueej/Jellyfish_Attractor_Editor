use jellyfish::dynamics::{Lorenz, generate_trajectory};
use jellyfish::math::Vec3;

#[test]
fn trajectory_generation() {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let initial = Vec3::new([1.0, 2.0, 3.0]);
    let trajectory = generate_trajectory(&system, initial, 0.01, 10);
    assert_eq!(trajectory.len(), 11);
    assert_eq!(trajectory[0], initial);
}
