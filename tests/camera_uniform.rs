use glam::Vec3;
use jellyfish::graphics::camera::OrbitCamera;
use jellyfish::graphics::camera_uniform::CameraUniform;

#[test]
fn camera_uniform_size() {
    assert_eq!(std::mem::size_of::<CameraUniform>(), 64)
}

#[test]
fn camera_uniform_matrix() {
    let camera = OrbitCamera::new(Vec3::ZERO, 80.0);
    let aspect = 16.0 / 9.0;
    let uniform = CameraUniform::new(&camera, aspect);
    let expected = camera.view_projection_matrix(aspect).to_cols_array_2d();
    assert_eq!(uniform.view_projectrion, expected);
}
