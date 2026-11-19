use jellyfish::dynamics::{Lorenz, generate_trajectory, rk4_step};
use jellyfish::math::{Bounds, Vec3};

fn main() {
    let system = Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    let initial = Vec3::new([1.0, 1.0, 1.0]);

    let trajectory = generate_trajectory(&system, initial, 0.01, 10_000, 1_000, rk4_step);
    println!("trajectory points: {}", trajectory.len());
    println!("last point: {:?}", trajectory.last().unwrap());

    let bounds = Bounds::from_points(&trajectory).unwrap();

    println!("Min: {:?}", bounds.min);
    println!("Max: {:?}", bounds.max);

    println!("Center of attractor: {:?}", bounds.center());
    println!("Size of attractor: {:?}", bounds.size());
}
