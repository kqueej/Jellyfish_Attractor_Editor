use jellyfish::graphics::projection::project_xy;

#[test]
fn project_xy_normalization() {
    let points = [[-10.0, -20.0, 5.0], [0.0, 0.0, 10.0], [10.0, 20.0, 15.0]];

    let vertices = project_xy(&points);
    assert_eq!(vertices.len(), 3);
    assert_eq!(vertices[1].position, [0.0, 0.0, 0.0]);
    for vertex in vertices {
        assert!(vertex.position[0].abs() <= 0.9);
        assert!(vertex.position[1].abs() <= 0.9);
        assert_eq!(vertex.position[2], 0.0);
    }
}

#[test]
fn normalize_xyz_preserves_depth() {
    use jellyfish::graphics::projection::normalize_xyz;

    let points = [[-1.0, -2.0, 1.0], [1.0, 2.0, 5.0]];

    let vertices = normalize_xyz(&points);
    assert_eq!(vertices.len(), 2);
    assert!((vertices[0].position[2] + 0.9).abs() < 1e-6);
    assert!((vertices[1].position[2] - 0.9).abs() < 1e-6);
}
