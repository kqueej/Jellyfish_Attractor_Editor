use super::DynamicalSystem;
use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Rossler {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl DynamicalSystem<3> for Rossler {
    fn derivative(&self, _time: f64, state: &Vec3) -> Vec3 {
        let [x, y, z] = *state.as_array();

        let dx = -y - z;
        let dy = x + self.a * y;
        let dz = self.b + z * (x - self.c);

        Vec3::new([dx, dy, dz])
    }
}
