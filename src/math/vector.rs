#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    coordinates: [f64; N],
}

impl<const N: usize> Vector<N> {
    // Constructor
    pub fn new(coordinates: [f64; N]) -> Self {
        Self { coordinates }
    }

    // link for array
    pub fn as_array(&self) -> &[f64; N] {
        &self.coordinates
    }

    // self array
    pub fn into_array(self) -> [f64; N] {
        self.coordinates
    }
}

pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;

// Test
#[cfg(test)]
mod tests {
    use super::*;

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
}
