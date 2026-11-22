use jellyfish::dynamics::{Attractor, Lorenz, Rossler};
use jellyfish::graphics::vertex::Vertex;
use jellyfish::math::Vec3;
use jellyfish::scene::attractor::generate_vertices;

fn check_vertices(vertices: &[Vertex]) {
    assert_eq!(vertices.len(), 1_001);

    for vertex in vertices {
        assert!(vertex.position.iter().all(|x| x.is_finite()));
        assert!(vertex.speed.is_finite());
        assert!((0.0..=1.0).contains(&vertex.speed));
    }

    assert!(vertices.iter().any(|vertex| vertex.speed > 0.0));
}

#[test]
fn lorenz_vertices_are_valid() {
    let system = Attractor::Lorenz(Lorenz {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    });

    let vertices = generate_vertices(&system, Vec3::new([1.0, 1.0, 1.0]), 0.01, 1_000, 1_000);

    check_vertices(&vertices);
}

#[test]
fn rossler_vertices_are_valid() {
    let system = Attractor::Rossler(Rossler {
        a: 0.2,
        b: 0.2,
        c: 5.7,
    });

    let vertices = generate_vertices(&system, Vec3::new([1.0, 0.0, 0.0]), 0.01, 1_000, 1_000);

    check_vertices(&vertices);
}
