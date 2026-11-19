use super::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds<const N: usize> {
    pub min: Vector<N>,
    pub max: Vector<N>,
}

impl<const N: usize> Bounds<N> {
    pub fn from_points(points: &[Vector<N>]) -> Option<Self> {
        let first = points.first()?;
        let mut min = *first.as_array();
        let mut max = *first.as_array();

        for point in points {
            for (i, &coordinate) in point.as_array().iter().enumerate() {
                min[i] = min[i].min(coordinate);
                max[i] = max[i].max(coordinate);
            }
        }

        Some(Self {
            min: Vector::new(min),
            max: Vector::new(max),
        })
    }
}
