use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    coordinates: [f64; N],
}

impl<const N: usize> Vector<N> {
    // constructor
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

// Composition
impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let coordinates: [f64; N] =
            std::array::from_fn(|i| self.coordinates[i] + rhs.coordinates[i]);

        Self::new(coordinates)
    }
}

// Subtract
impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let coordinates: [f64; N] =
            std::array::from_fn(|i: usize| self.coordinates[i] - rhs.coordinates[i]);
        Self::new(coordinates)
    }
}

// Vector multiplication
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let coordinates: [f64; N] = std::array::from_fn(|i: usize| self.coordinates[i] * rhs);
        Self::new(coordinates)
    }
}

// Division
impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let coordinates: [f64; N] = std::array::from_fn(|i: usize| self.coordinates[i] / rhs);
        Self::new(coordinates)
    }
}
