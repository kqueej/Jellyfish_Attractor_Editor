use super::attractor::generate_vertices;
use crate::dynamics::{Attractor, Lorenz, Rossler};
use crate::graphics::vertex::Vertex;
use crate::math::Vec3;

pub struct AttractorConfig {
    pub system: Attractor,
    pub initial: Vec3,
    pub step: f64,
    pub count: usize,
    pub warmup: usize,
}

impl AttractorConfig {
    pub fn lorenz() -> Self {
        Self {
            system: Attractor::Lorenz(Lorenz {
                sigma: 10.0,
                rho: 28.0,
                beta: 8.0 / 3.0,
            }),
            initial: Vec3::new([1.0, 1.0, 1.0]),
            step: 0.01,
            count: 10_000,
            warmup: 1_000,
        }
    }

    pub fn rossler() -> Self {
        Self {
            system: Attractor::Rossler(Rossler {
                a: 0.2,
                b: 0.2,
                c: 5.7,
            }),
            initial: Vec3::new([1.0, 0.0, 0.0]),
            step: 0.01,
            count: 50_000,
            warmup: 2_000,
        }
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        generate_vertices(
            &self.system,
            self.initial,
            self.step,
            self.count,
            self.warmup,
        )
    }
}
