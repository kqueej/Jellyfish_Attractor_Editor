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

#[test]
fn camera_zoom_direction() {
    let mut camera = OrbitCamera::new(Vec3::ZERO, 5.0);

    camera.zoom(1.0);
    assert!(camera.distance < 5.0);

    camera.zoom(-1.0);
    assert!((camera.distance - 5.0).abs() < 1e-5);
}

#[test]
fn camera_zoom_limits() {
    let mut camera = OrbitCamera::new(Vec3::ZERO, 5.0);

    camera.zoom(1000.0);
    assert_eq!(camera.distance, 0.01);

    camera.zoom(-1000.0);
    assert_eq!(camera.distance, 20.0);
}

#[test]
fn camera_pan_preserves_distance() {
    let mut camera = OrbitCamera::new(Vec3::ZERO, 5.0);

    let old_target = camera.target;
    let old_position = camera.position();

    camera.pan(100.0, -50.0, 900.0);

    assert_ne!(camera.target, old_target);

    let target_movement = camera.target - old_target;
    let camera_movement = camera.position() - old_position;

    assert!((target_movement - camera_movement).length() < 1e-5);
    assert!((camera.position().distance(camera.target) - 5.0).abs() < 1e-5);
}
