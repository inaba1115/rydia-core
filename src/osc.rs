use pyo3::prelude::*;
use std::f32::consts::TAU;

#[pyclass]
pub struct SinOsc {
    #[pyo3(get, set)]
    pub sample_rate: f32,

    #[pyo3(get, set)]
    pub phase: f32,
}

#[pymethods]
impl SinOsc {
    #[new]
    #[pyo3(signature = (sample_rate, phase=0.0))]
    pub fn new(sample_rate: f32, phase: f32) -> Self {
        SinOsc {
            sample_rate: sample_rate,
            phase: phase,
        }
    }

    pub fn process(&mut self, frequency: f32) -> f32 {
        let xn = (self.phase * TAU).sin();
        let phase_delta = frequency / self.sample_rate;

        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        xn
    }
}

#[pyclass]
pub struct WhiteNoise {}

#[pymethods]
impl WhiteNoise {
    #[new]
    pub fn new() -> Self {
        WhiteNoise {}
    }

    pub fn process(&mut self) -> f32 {
        rand::random_range(-1.0..=1.0)
    }
}
