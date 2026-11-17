use jellyfish::math::{Vec2, Vec3};

#[test]
// for Vec2
fn vector2_creation() {
    let vector = Vec2::new([1.0, 2.0]);
    assert_eq!(vector.as_array(), &[1.0, 2.0]);
}

#[test]
// for Vec3
fn vector3_creation() {
    let vector = Vec3::new([1.0, 2.0, 3.0]);
    assert_eq!(vector.as_array(), &[1.0, 2.0, 3.0]);
}

// for as_array
#[test]
fn vector_as_array() {
    let vector = Vec3::new([1.0, 2.0, 3.0]);
    let coordinates = vector.as_array();
    assert_eq!(coordinates, &[1.0, 2.0, 3.0]);
}

// for into_array
#[test]
fn vector2_into_array() {
    let vector = Vec2::new([1.0, 2.0]);
    let coordinates = vector.into_array();
    assert_eq!(coordinates, [1.0, 2.0]);
}

#[test]
fn vector3_into_array() {
    let vector = Vec3::new([1.0, 2.0, 3.0]);
    let coordinates = vector.into_array();
    assert_eq!(coordinates, [1.0, 2.0, 3.0]);
}

// for 3 vectors
#[test]
fn vector_equality() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    let b = Vec3::new([1.0, 2.0, 3.0]);
    let c = Vec3::new([1.0, 2.0, 4.0]);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// For vector addition
#[test]
fn vector3_addition() {
    let a = Vec3::new([1.0, 2.0, 3.0]);
    let b = Vec3::new([4.0, 5.0, 6.0]);
    let result = a + b;

    assert_eq!(result.as_array(), &[5.0, 7.0, 9.0]);
}

#[test]
fn vector2_addition() {
    let a = Vec2::new([1.0, 2.0]);
    let b = Vec2::new([3.0, 4.0]);
    let result = a + b;

    assert_eq!(result.as_array(), &[4.0, 6.0]);
}

// For subtraction
#[test]
fn vector2_subtraction() {
    let a = Vec2::new([1.0, 2.0]);
    let b = Vec2::new([3.0, 4.0]);
    let result = a - b;

    assert_eq!(result.as_array(), &[-2.0, -2.0]);
}

#[test]
fn vector3_subtraction() {
    let a = Vec3::new([8.0, 7.0, 6.0]);
    let b = Vec3::new([3.0, 2.0, 1.0]);
    let result = a - b;

    assert_eq!(result.as_array(), &[5.0, 5.0, 5.0]);
}

// For vector multiplication
#[test]
fn vector2_multiplication() {
    let vector = Vec2::new([2.0, 3.0]);
    let result = vector * 4.0;

    assert_eq!(result.as_array(), &[8.0, 12.0]);
}

#[test]
fn vector3_multiplication() {
    let vector = Vec3::new([1.0, 2.0, 3.0]);
    let result = vector * 5.0;

    assert_eq!(result.as_array(), &[5.0, 10.0, 15.0]);
}
// Division
#[test]
fn vector2_division() {
    let vector = Vec2::new([8.0, 12.0]);
    let result = vector / 4.0;

    assert_eq!(result.as_array(), &[2.0, 3.0]);
}

#[test]
fn vector3_division() {
    let vector = Vec3::new([10.0, 20.0, 30.0]);
    let result = vector / 5.0;

    assert_eq!(result.as_array(), &[2.0, 4.0, 6.0]);
}
