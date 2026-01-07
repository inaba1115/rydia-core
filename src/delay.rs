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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32, eps: f32) {
        assert!(
            (a - b).abs() <= eps,
            "not approx equal: a={a}, b={b}, |a-b|={}",
            (a - b).abs()
        );
    }

    #[test]
    fn test_calc_fb_monotonicity() {
        let delay = 1.0;

        let fb_short = calc_fb(delay, 10.0);
        let fb_mid = calc_fb(delay, 100.0);
        let fb_long = calc_fb(delay, 1000.0);

        // decay が長いほどフィードバックは 1 に近づく
        assert!(fb_short < fb_mid);
        assert!(fb_mid < fb_long);
        assert!(fb_long < 1.0);
    }

    #[test]
    fn test_delay_s_basic_behavior() {
        let mut d = DelayS::new(4);

        // 最初は必ず 0 が出る
        approx(d.process(10.0, 2), 0.0, 1e-6);
        approx(d.process(11.0, 2), 0.0, 1e-6);

        // delay 分遅れて入力が出てくる
        approx(d.process(12.0, 2), 10.0, 1e-6);
        approx(d.process(13.0, 2), 11.0, 1e-6);
    }

    #[test]
    fn test_delay_n_truncates_fractional_delay() {
        let mut d = DelayN::new(1.0, 4.0);

        approx(d.process(10.0, 2.1), 0.0, 1e-6);
        approx(d.process(11.0, 2.1), 0.0, 1e-6);

        // floor(delay_sec * sr) が使われる
        approx(d.process(12.0, 2.1), 10.0, 1e-6);
        approx(d.process(13.0, 2.1), 11.0, 1e-6);
    }

    #[test]
    fn test_delay_l_interpolates_smoothly() {
        let mut d = DelayL::new(1.0, 4.0);

        let y0 = d.process(10.0, 2.5);
        let y1 = d.process(11.0, 2.5);
        let y2 = d.process(12.0, 2.5);

        // 最初はゼロ
        approx(y0, 0.0, 1e-6);

        // 補間された値が徐々に立ち上がる
        assert!(y1 > 0.0);
        assert!(y2 > y1);
    }

    #[test]
    fn test_comb_s_energy_builds_up() {
        let mut c = CombS::new(4);

        let mut y_prev = 0.0;
        for i in 0..10 {
            let y = c.process(1.0, 2, 10);
            if i > 2 {
                // フィードバックによりエネルギーが蓄積
                assert!(y >= y_prev);
            }
            y_prev = y;
        }
    }

    #[test]
    fn test_comb_n_and_l_do_not_produce_nan_or_inf() {
        let mut c_n = CombN::new(1.0, 4.0);
        let mut c_l = CombL::new(1.0, 4.0);

        for _ in 0..1000 {
            let y_n = c_n.process(1.0, 2.3, 5.0);
            let y_l = c_l.process(1.0, 2.3, 5.0);

            assert!(y_n.is_finite(), "CombN produced non-finite value");
            assert!(y_l.is_finite(), "CombL produced non-finite value");
        }
    }

    #[test]
    fn test_allpass_preserves_signal_energy_trend() {
        let mut ap = AllpassL::new(1.0, 4.0);

        // 単位インパルス
        let y0 = ap.process(1.0, 2.0, 10.0);
        let y1 = ap.process(0.0, 2.0, 10.0);
        let y2 = ap.process(0.0, 2.0, 10.0);

        // allpass なのでエネルギーは分散するが消えない
        assert!(y0.abs() > 0.0);
        assert!(y1.abs() > 0.0 || y2.abs() > 0.0);
    }

    #[test]
    fn test_allpass_does_not_produce_nan_or_inf() {
        let mut ap = AllpassN::new(1.0, 4.0);

        for _ in 0..1000 {
            let y = ap.process(0.5, 2.0, 5.0);
            assert!(y.is_finite(), "Allpass produced non-finite value");
        }
    }
}
