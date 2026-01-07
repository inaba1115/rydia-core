use crate::ring_buffer::{RingBufferL, RingBufferN, RingBufferS};
use pyo3::prelude::*;

/// Feedback coefficient used by SuperCollider Comb / Allpass
#[pyfunction]
pub fn calc_fb(delay: f32, decay: f32) -> f32 {
    // fb == 0.001 ** (delay / abs(decay)) * sign(decay)
    0.001_f32.powf(delay / decay.abs()) * decay.signum()
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct DelayS {
    buf: RingBufferS,
}

#[pymethods]
impl DelayS {
    #[new]
    pub fn new(max_delay_samp: usize) -> Self {
        Self {
            buf: RingBufferS::new(max_delay_samp),
        }
    }

    pub fn process(&mut self, xn: f32, delay_samp: usize) -> f32 {
        let yn = self.buf.read(delay_samp);
        self.buf.write(xn);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct DelayN {
    buf: RingBufferN,
}

#[pymethods]
impl DelayN {
    #[new]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        Self {
            buf: RingBufferN::new(sample_rate, max_delay_sec),
        }
    }

    pub fn process(&mut self, xn: f32, delay_sec: f32) -> f32 {
        let yn = self.buf.read(delay_sec);
        self.buf.write(xn);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct DelayL {
    buf: RingBufferL,
}

#[pymethods]
impl DelayL {
    #[new]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        Self {
            buf: RingBufferL::new(sample_rate, max_delay_sec),
        }
    }

    pub fn process(&mut self, xn: f32, delay_sec: f32) -> f32 {
        let yn = self.buf.read(delay_sec);
        self.buf.write(xn);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct CombS {
    buf: RingBufferS,
}

#[pymethods]
impl CombS {
    #[new]
    pub fn new(max_delay_samp: usize) -> Self {
        Self {
            buf: RingBufferS::new(max_delay_samp),
        }
    }

    pub fn process(&mut self, xn: f32, delay_samp: usize, decay_samp: usize) -> f32 {
        let yn = self.buf.read(delay_samp);
        let fb = calc_fb(delay_samp as f32, decay_samp as f32);
        self.buf.write(xn + yn * fb);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct CombN {
    buf: RingBufferN,
}

#[pymethods]
impl CombN {
    #[new]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        Self {
            buf: RingBufferN::new(sample_rate, max_delay_sec),
        }
    }

    pub fn process(&mut self, xn: f32, delay_sec: f32, decay_sec: f32) -> f32 {
        let yn = self.buf.read(delay_sec);
        let fb = calc_fb(delay_sec, decay_sec);
        self.buf.write(xn + yn * fb);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct CombL {
    buf: RingBufferL,
}

#[pymethods]
impl CombL {
    #[new]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        Self {
            buf: RingBufferL::new(sample_rate, max_delay_sec),
        }
    }

    pub fn process(&mut self, xn: f32, delay_sec: f32, decay_sec: f32) -> f32 {
        let yn = self.buf.read(delay_sec);
        let fb = calc_fb(delay_sec, decay_sec);
        self.buf.write(xn + yn * fb);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct AllpassS {
    buf: RingBufferS,
}

#[pymethods]
impl AllpassS {
    #[new]
    pub fn new(max_delay_samp: usize) -> Self {
        Self {
            buf: RingBufferS::new(max_delay_samp),
        }
    }

    pub fn process(&mut self, xn: f32, delay_samp: usize, decay_samp: usize) -> f32 {
        let k = calc_fb(delay_samp as f32, decay_samp as f32);
        let s_delay = self.buf.read(delay_samp);

        let sn = xn + k * s_delay;
        let yn = -k * sn + s_delay;

        self.buf.write(sn);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct AllpassN {
    buf: RingBufferN,
}

#[pymethods]
impl AllpassN {
    #[new]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        Self {
            buf: RingBufferN::new(sample_rate, max_delay_sec),
        }
    }

    pub fn process(&mut self, xn: f32, delay_sec: f32, decay_sec: f32) -> f32 {
        let k = calc_fb(delay_sec, decay_sec);
        let s_delay = self.buf.read(delay_sec);

        let sn = xn + k * s_delay;
        let yn = -k * sn + s_delay;

        self.buf.write(sn);
        yn
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct AllpassL {
    buf: RingBufferL,
}

#[pymethods]
impl AllpassL {
    #[new]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        Self {
            buf: RingBufferL::new(sample_rate, max_delay_sec),
        }
    }

    pub fn process(&mut self, xn: f32, delay_sec: f32, decay_sec: f32) -> f32 {
        let k = calc_fb(delay_sec, decay_sec);
        let s_delay = self.buf.read(delay_sec);

        let sn = xn + k * s_delay;
        let yn = -k * sn + s_delay;

        self.buf.write(sn);
        yn
    }
}
