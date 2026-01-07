use pyo3::prelude::*;
use std::f32::consts::PI;

const B: f32 = 4.0 / PI;
const C: f32 = -4.0 / (PI * PI);
const P: f32 = 0.225;

fn parabolic_sine(x: f32) -> f32 {
    let y1 = B * x + C * x * x.abs();
    P * (y1 * y1.abs() - y1) + y1
}

fn unipolar_to_bipolar(x: f32) -> f32 {
    x * 2.0 - 1.0
}

#[derive(Clone, Copy)]
enum LfoWaveform {
    Sine,
    Triangle,
    Saw,
}

impl From<usize> for LfoWaveform {
    fn from(v: usize) -> Self {
        match v {
            0 => LfoWaveform::Sine,
            1 => LfoWaveform::Triangle,
            _ => LfoWaveform::Saw,
        }
    }
}

#[pyclass]
pub struct Lfo {
    sample_rate: f32,
    waveform: LfoWaveform,
    phase: f32,
    phase_qp: f32,
}

#[pymethods]
impl Lfo {
    #[new]
    #[pyo3(signature = (sample_rate, waveform = 0))]
    pub fn new(sample_rate: f32, waveform: usize) -> Self {
        assert!(sample_rate > 0.0);
        Self {
            sample_rate,
            waveform: waveform.into(),
            phase: 0.0,
            phase_qp: 0.25,
        }
    }

    pub fn process(&mut self, frequency: f32) -> (f32, f32) {
        let phase_inc = frequency / self.sample_rate;

        let tmp = unipolar_to_bipolar(self.phase);
        let tmp_qp = unipolar_to_bipolar(self.phase_qp);

        let (y, y_qp) = match self.waveform {
            LfoWaveform::Sine => {
                let angle = tmp * PI;
                let angle_qp = tmp_qp * PI;
                (parabolic_sine(-angle), parabolic_sine(-angle_qp))
            }
            LfoWaveform::Triangle => (2.0 * tmp.abs() - 1.0, 2.0 * tmp_qp.abs() - 1.0),
            LfoWaveform::Saw => (tmp, tmp_qp),
        };

        self.phase += phase_inc;
        self.phase -= self.phase.floor();

        self.phase_qp += phase_inc;
        self.phase_qp -= self.phase_qp.floor();

        (y, y_qp)
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.phase_qp = 0.25;
    }

    #[getter]
    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
