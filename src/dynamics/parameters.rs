use super::Attractor;

#[derive(Debug, Clone, Copy)]
pub struct ParameterSpec {
    pub name: &'static str,
    pub min: f64,
    pub max: f64,
}

pub const LORENZ_PARAMETERS: [ParameterSpec; 3] = [
    ParameterSpec {
        name: "σ (Sigma)",
        min: 1.0,
        max: 30.0,
    },
    ParameterSpec {
        name: "ρ (Rho)",
        min: 0.0,
        max: 60.0,
    },
    ParameterSpec {
        name: "β (Beta)",
        min: 0.1,
        max: 6.0,
    },
];

pub const ROSSLER_PARAMETERS: [ParameterSpec; 3] = [
    ParameterSpec {
        name: "a",
        min: 0.05,
        max: 1.0,
    },
    ParameterSpec {
        name: "b",
        min: 0.05,
        max: 1.0,
    },
    ParameterSpec {
        name: "c",
        min: 1.0,
        max: 20.0,
    },
];

impl Attractor {
    pub fn parameter_specs(&self) -> &'static [ParameterSpec] {
        match self {
            Self::Lorenz(_) => &LORENZ_PARAMETERS,
            Self::Rossler(_) => &ROSSLER_PARAMETERS,
        }
    }

    pub fn parameter_value(&self, index: usize) -> Option<f64> {
        match self {
            Self::Lorenz(system) => match index {
                0 => Some(system.sigma),
                1 => Some(system.rho),
                2 => Some(system.beta),
                _ => None,
            },

            Self::Rossler(system) => match index {
                0 => Some(system.a),
                1 => Some(system.b),
                2 => Some(system.c),
                _ => None,
            },
        }
    }

    pub fn set_parameter(&mut self, index: usize, value: f64) -> bool {
        let Some(spec) = self.parameter_specs().get(index) else {
            return false;
        };

        if !value.is_finite() || value < spec.min || value > spec.max {
            return false;
        }

        match self {
            Self::Lorenz(system) => match index {
                0 => system.sigma = value,
                1 => system.rho = value,
                2 => system.beta = value,
                _ => return false,
            },

            Self::Rossler(system) => match index {
                0 => system.a = value,
                1 => system.b = value,
                2 => system.c = value,
                _ => return false,
            },
        }

        true
    }
}
