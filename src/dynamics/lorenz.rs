use super::DynamicalSystem;
use crate::math::Vec3;

impl DynamicalSystem<3> for Lorenz {
    fn derivative(&self, _time: f64, state: &Vec3) -> Vec3 {
        let [x, y, z] = *state.as_array();
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        Vec3::new([dx, dy, dz])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lorenz {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}
