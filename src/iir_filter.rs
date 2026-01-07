use pyo3::prelude::*;
use std::f32::consts::TAU;

// filter coefs
pub const A0: usize = 0;
pub const A1: usize = 1;
pub const A2: usize = 2;
pub const B1: usize = 3;
pub const B2: usize = 4;
pub const N_COEFS: usize = 5;

// filter states
pub const X_Z1: usize = 0;
pub const X_Z2: usize = 1;
pub const Y_Z1: usize = 2;
pub const Y_Z2: usize = 3;
pub const N_STATES: usize = 4;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Biquad {
    #[pyo3(get, set)]
    pub coefs: Vec<f32>,

    #[pyo3(get, set)]
    pub states: Vec<f32>,
}

#[pymethods]
impl Biquad {
    #[new]
    #[pyo3(signature = (coefs))]
    pub fn new(coefs: Vec<f32>) -> Self {
        Biquad {
            coefs: coefs,
            states: vec![0.0; N_STATES],
        }
    }

    pub fn process(&mut self, xn: f32) -> f32 {
        // TRANSPOSE CANONICAL
        let yn = self.coefs[A0] * xn + self.states[X_Z1];
        self.states[X_Z1] = self.coefs[A1] * xn - self.coefs[B1] * yn + self.states[X_Z2];
        self.states[X_Z2] = self.coefs[A2] * xn - self.coefs[B2] * yn;
        yn
    }
}

#[pyclass]
pub struct IirLpf1 {
    #[pyo3(get, set)]
    pub sample_rate: f32,

    #[pyo3(get, set)]
    pub biquad: Biquad,

    #[pyo3(get, set)]
    pub fc: f32,
}

#[pymethods]
impl IirLpf1 {
    #[new]
    #[pyo3(signature = (sample_rate))]
    pub fn new(sample_rate: f32) -> Self {
        let coefs = vec![0.0; N_COEFS];

        IirLpf1 {
            sample_rate: sample_rate,
            biquad: Biquad::new(coefs),
            fc: 0.0,
        }
    }

    fn update_coefs(&mut self) {
        let theta_c = TAU * self.fc / self.sample_rate;
        let gamma = theta_c.cos() / (1.0 + theta_c.sin());

        let mut coefs: Vec<f32> = vec![0.0; N_COEFS];
        coefs[A0] = (1.0 - gamma) / 2.0;
        coefs[A1] = (1.0 - gamma) / 2.0;
        coefs[B1] = -gamma;

        self.biquad.coefs = coefs;
    }

    pub fn process(&mut self, xn: f32, fc: f32) -> f32 {
        if fc != self.fc {
            self.fc = fc;
            self.update_coefs();
        }

        self.biquad.process(xn)
    }
}

#[pyclass]
pub struct IirLpf2 {
    #[pyo3(get, set)]
    pub sample_rate: f32,

    #[pyo3(get, set)]
    pub biquad: Biquad,

    #[pyo3(get, set)]
    pub fc: f32,

    #[pyo3(get, set)]
    pub q: f32,
}

#[pymethods]
impl IirLpf2 {
    #[new]
    #[pyo3(signature = (sample_rate))]
    pub fn new(sample_rate: f32) -> Self {
        let coefs = vec![0.0; N_COEFS];

        IirLpf2 {
            sample_rate: sample_rate,
            biquad: Biquad::new(coefs),
            fc: 0.0,
            q: 0.0,
        }
    }

    fn update_coefs(&mut self) {
        let theta_c = TAU * self.fc / self.sample_rate;
        let d = 1.0 / self.q;
        let beta_numerator = 1.0 - (d / 2.0) * theta_c.sin();
        let beta_denominator = 1.0 + (d / 2.0) * theta_c.sin();
        let beta = 0.5 * (beta_numerator / beta_denominator);
        let gamma = (0.5 + beta) * theta_c.cos();
        let alpha = (0.5 + beta - gamma) / 2.0;

        let mut coefs: Vec<f32> = vec![0.0; N_COEFS];
        coefs[A0] = alpha;
        coefs[A1] = 2.0 * alpha;
        coefs[A2] = alpha;
        coefs[B1] = -2.0 * gamma;
        coefs[B2] = 2.0 * beta;

        self.biquad.coefs = coefs;
    }

    pub fn process(&mut self, xn: f32, fc: f32, q: f32) -> f32 {
        if fc != self.fc || q != self.q {
            self.fc = fc;
            self.q = q;
            self.update_coefs();
        }

        self.biquad.process(xn)
    }
}
