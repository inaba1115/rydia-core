use pyo3::prelude::*;
use std::f32::consts::TAU;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[pyclass]
pub struct SinOsc {
    sample_rate: f32,
    phase: f32,
}

#[pymethods]
impl SinOsc {
    #[new]
    #[pyo3(signature = (sample_rate, phase = 0.0))]
    pub fn new(sample_rate: f32, phase: f32) -> Self {
        assert!(sample_rate > 0.0);
        Self { sample_rate, phase }
    }

    pub fn process(&mut self, frequency: f32) -> f32 {
        // output at current phase
        let y = (self.phase * TAU).sin();

        // advance phase
        self.phase += frequency / self.sample_rate;
        self.phase -= self.phase.floor();

        y
    }

    /// Reset phase to 0.0
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    #[getter]
    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    #[getter]
    fn phase(&self) -> f32 {
        self.phase
    }
}

#[pyclass]
pub struct WhiteNoise {
    rng: SmallRng,
}

#[pymethods]
impl WhiteNoise {
    #[new]
    pub fn new() -> Self {
        Self {
            rng: SmallRng::from_os_rng(),
        }
    }

    pub fn process(&mut self) -> f32 {
        self.rng.random_range(-1.0..=1.0)
    }
}
