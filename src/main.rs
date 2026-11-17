use jellyfish::math::{Vec2, Vec3};

fn main() {
    let point_2d = Vec2::new([1.0, 2.0]);
    let point_3d = Vec3::new([1.0, 2.0, 3.0]);

    println!("2D: {:?}", point_2d);
    println!("3D: {:?}", point_3d);

    println!("2D coordinates: {:?}", point_2d.as_array());
    println!("3D coordinates: {:?}", point_3d.as_array());
}
