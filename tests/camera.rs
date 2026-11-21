use glam::Vec3;
use jellyfish::graphics::camera::OrbitCamera;

#[test]
fn camer_distance() {
    let camera = OrbitCamera::new(Vec3::ZERO, 80.0);
    let distance = camera.position().distance(camera.target);
    assert!((distance - 80.0).abs() < 1e-4);
}

#[test]
fn camer_target_is_centered() {
    let target = Vec3::new(1.0, 2.0, 3.0);
    let camera = OrbitCamera::new(target, 80.0);
    let matrix = camera.view_projection_matrix(16.0 / 9.0);
    let clip = matrix * target.extend(1.0);
    assert!(clip.w > 0.0);
    assert!((clip.x / clip.w).abs() < 1e-5);
    assert!((clip.y / clip.w).abs() < 1e-5);
}

#[test]
fn camera_rotation_limits_pitch() {
    let mut camera = OrbitCamera::new(Vec3::ZERO, 5.0);

    camera.rotate(0.0, 100_000.0);

    assert!(camera.pitch < std::f32::consts::FRAC_PI_2);

    camera.rotate(0.0, -200_000.0);

    assert!(camera.pitch > -std::f32::consts::FRAC_PI_2);
}
