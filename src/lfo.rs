use pyo3::prelude::*;
use std::f32::consts::PI;

const B: f32 = 4.0 / PI;
const C: f32 = -4.0 / (PI * PI);
const P: f32 = 0.225;

fn parabolic_sine(x: f32) -> f32 {
    let y1 = B * x + C * x * x.abs();
    let y2 = P * (y1 * y1.abs() - y1) + y1;
    return y2;
}

fn unipolar_to_bipolar(x: f32) -> f32 {
    x * 2.0 - 1.0
}

#[pyclass]
pub struct Lfo {
    #[pyo3(get, set)]
    pub sample_rate: f32,

    #[pyo3(get, set)]
    pub waveform: usize,

    #[pyo3(get, set)]
    pub mod_counter: f32,

    #[pyo3(get, set)]
    pub mod_counter_qp: f32,
}

#[pymethods]
impl Lfo {
    #[new]
    #[pyo3(signature = (sample_rate, waveform=0))]
    pub fn new(sample_rate: f32, waveform: usize) -> Self {
        Lfo {
            sample_rate: sample_rate,
            waveform: waveform,
            mod_counter: 0.0,
            mod_counter_qp: 0.25,
        }
    }

    pub fn process(&mut self, frequency: f32) -> (f32, f32) {
        let phase_inc = frequency / self.sample_rate;

        let tmp = unipolar_to_bipolar(self.mod_counter);
        let tmp_qp = unipolar_to_bipolar(self.mod_counter_qp);

        let (y, y_qp) = match self.waveform {
            0 => {
                // sin
                let angle = tmp * PI;
                let angle_qp = tmp_qp * PI;
                let y = parabolic_sine(-angle);
                let y_qp = parabolic_sine(-angle_qp);
                (y, y_qp)
            }
            1 => {
                // tri
                let y = 2.0 * tmp.abs() - 1.0;
                let y_qp = 2.0 * tmp_qp.abs() - 1.0;
                (y, y_qp)
            }
            _ => {
                // saw
                (tmp, tmp_qp)
            }
        };

        self.mod_counter += phase_inc;
        if self.mod_counter >= 1.0 {
            self.mod_counter -= 1.0;
        }

        self.mod_counter_qp += phase_inc;
        if self.mod_counter_qp >= 1.0 {
            self.mod_counter_qp -= 1.0;
        }

        (y, y_qp)
    }
}
