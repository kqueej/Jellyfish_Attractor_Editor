
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize>
{
    coordinates: [f64; N],
}

impl<const N: usize> Vector<N>
{
    // Constructor 
    pub fn new(coordinates: [f64; N]) -> Self
    {
        Self {
            coordinates,
        }
    }

    // link for array
    pub fn as_array(&self) -> &[f64; N]
    {
        &self.coordinates
    }

    // self array
    pub fn into_array(self) -> [f64; N]
    {
        self.coordinates
    }
}


pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;
