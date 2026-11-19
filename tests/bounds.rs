use jellyfish::math::{Bounds, Vec2, Vec3};

#[test]
fn bounds_3d() {
    let points = [
        Vec3::new([1.0, 5.0, -2.0]),
        Vec3::new([-3.0, 2.0, 7.0]),
        Vec3::new([4.0, -1.0, 3.0]),
    ];

    let bounds = Bounds::from_points(&points).unwrap();
    assert_eq!(bounds.min, Vec3::new([-3.0, -1.0, -2.0]));
    assert_eq!(bounds.max, Vec3::new([4.0, 5.0, 7.0]));
}

#[test]
fn bounds_2d() {
    let points = [Vec2::new([-1.0, -4.0]), Vec2::new([2.0, 6.0])];

    let bounds = Bounds::from_points(&points).unwrap();
    assert_eq!(bounds.min, Vec2::new([-1.0, -4.0]));
    assert_eq!(bounds.max, Vec2::new([2.0, 6.0]));
}

#[test]
fn bounds_empty() {
    let points: [Vec3; 0] = [];

    assert_eq!(Bounds::from_points(&points), None);
}
