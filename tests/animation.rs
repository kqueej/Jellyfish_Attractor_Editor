use jellyfish::animation::AnimationController;

#[test]
fn initial_state() {
    let animation = AnimationController::new(10_000);

    assert_eq!(animation.visible_vertices(), 2);
    assert_eq!(animation.speed(), 5000.0);
}

#[test]
fn animation_progress() {
    let mut animation = AnimationController::new(10_000);

    animation.update(1.0);

    assert_eq!(animation.visible_vertices(), 5002);
}

#[test]
fn pause_and_resume() {
    let mut animation = AnimationController::new(10_000);

    animation.toggle_pause();
    animation.update(1.0);

    assert_eq!(animation.visible_vertices(), 2);

    animation.toggle_pause();
    animation.update(1.0);

    assert_eq!(animation.visible_vertices(), 5002);
}

#[test]
fn reset_animation() {
    let mut animation = AnimationController::new(10_000);

    animation.update(1.0);
    animation.toggle_pause();
    animation.reset();

    assert_eq!(animation.visible_vertices(), 2);

    animation.update(1.0);

    assert_eq!(animation.visible_vertices(), 5002);
}

#[test]
fn speed_control() {
    let mut animation = AnimationController::new(10_000);

    animation.speed_up();
    assert_eq!(animation.speed(), 6250.0);

    animation.slow_down();
    assert_eq!(animation.speed(), 5000.0);
}

#[test]
fn speed_limits() {
    let mut animation = AnimationController::new(10_000);

    for _ in 0..100 {
        animation.speed_up();
    }

    assert_eq!(animation.speed(), 100_000.0);

    for _ in 0..100 {
        animation.slow_down();
    }

    assert_eq!(animation.speed(), 100.0);
}

#[test]
fn trajectory_limit() {
    let mut animation = AnimationController::new(1000);

    animation.update(100.0);

    assert_eq!(animation.visible_vertices(), 1000);
}
