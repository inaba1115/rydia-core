use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct LeakDc {
    #[pyo3(get, set)]
    pub coef: f32,

    x1: f32,
    y1: f32,
}

#[pymethods]
impl LeakDc {
    #[new]
    #[pyo3(signature = (coef = 0.995))]
    pub fn new(coef: f32) -> Self {
        LeakDc {
            coef,
            x1: 0.0,
            y1: 0.0,
        }
    }

    /// Process one sample.
    ///
    /// y[n] = x[n] - x[n-1] + coef * y[n-1]
    pub fn process(&mut self, xn: f32) -> f32 {
        let yn = xn - self.x1 + self.coef * self.y1;
        self.x1 = xn;
        self.y1 = yn;
        yn
    }

    /// Reset internal state.
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
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
    fn leak_dc_initial_output_is_zero() {
        let mut dc = LeakDc::new(0.995);
        let y = dc.process(0.0);
        approx(y, 0.0, 1e-6);
    }

    #[test]
    fn leak_dc_removes_constant_dc() {
        let mut dc = LeakDc::new(0.995);

        let mut y = 0.0;
        for _ in 0..10_000 {
            y = dc.process(1.0);
        }

        // DC 成分は最終的にほぼ 0 に近づく
        assert!(y.abs() < 1e-3, "dc not sufficiently removed: y={y}");
    }

    #[test]
    fn leak_dc_allows_signal_changes() {
        let mut dc = LeakDc::new(0.995);

        // ステップ入力
        let y0 = dc.process(0.0);
        let y1 = dc.process(1.0);
        let y2 = dc.process(1.0);

        approx(y0, 0.0, 1e-6);
        assert!(y1 > 0.0);
        assert!(y2 < y1); // 徐々に減衰
    }

    #[test]
    fn leak_dc_reset_clears_state() {
        let mut dc = LeakDc::new(0.995);

        dc.process(1.0);
        dc.process(1.0);

        dc.reset();

        let y = dc.process(0.0);
        approx(y, 0.0, 1e-6);
    }

    #[test]
    fn leak_dc_does_not_produce_nan_or_inf() {
        let mut dc = LeakDc::new(0.999);

        for _ in 0..100_000 {
            let y = dc.process(0.5);
            assert!(y.is_finite(), "non-finite output: {y}");
        }
    }
}
