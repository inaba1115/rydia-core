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

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    #[test]
    fn sinosc_output_is_bounded() {
        let mut osc = SinOsc::new(SR, 0.0);
        for _ in 0..10_000 {
            let y = osc.process(440.0);
            assert!(y >= -1.0 && y <= 1.0, "output out of range: {y}");
        }
    }

    #[test]
    fn sinosc_phase_is_always_normalized() {
        let mut osc = SinOsc::new(SR, 0.0);

        // 極端な周波数でも破綻しないこと
        for _ in 0..1_000 {
            osc.process(SR * 10.0);
            assert!(
                osc.phase >= 0.0 && osc.phase < 1.0,
                "phase not normalized: {}",
                osc.phase
            );
        }
    }

    #[test]
    fn sinosc_is_periodic_for_integer_period() {
        let freq = 1_000.0;
        let samples_per_period = (SR / freq) as usize;

        let mut osc = SinOsc::new(SR, 0.0);

        let y0 = osc.process(freq);

        // Advance phase by exactly one period
        for _ in 0..(samples_per_period - 1) {
            osc.process(freq);
        }

        // This output should match the initial phase (mod 1.0)
        let y1 = osc.process(freq);

        // 位相周期性（誤差許容）
        assert!(
            (y0 - y1).abs() < 1e-4,
            "periodicity broken: y0={y0}, y1={y1}"
        );
    }

    #[test]
    fn sinosc_reset_sets_phase_to_zero() {
        let mut osc = SinOsc::new(SR, 0.3);

        // 一度進める
        osc.process(440.0);
        assert!(osc.phase != 0.0);

        // reset
        osc.reset();
        assert!(
            (osc.phase - 0.0).abs() < f32::EPSILON,
            "phase not reset: {}",
            osc.phase
        );

        // reset 後の最初の出力は sin(0) = 0
        let y = osc.process(440.0);
        assert!(y.abs() < 1e-6, "output after reset not zero: {y}");
    }

    #[test]
    fn sinosc_does_not_produce_nan_or_inf() {
        let mut osc = SinOsc::new(SR, 0.0);

        let freqs = [0.0, 1.0, 440.0, SR / 2.0, SR, SR * 10.0];

        for &f in &freqs {
            for _ in 0..100 {
                let y = osc.process(f);
                assert!(y.is_finite(), "non-finite output: {y}");
            }
        }
    }

    #[test]
    fn white_noise_is_bounded_and_mean_is_near_zero() {
        let mut noise = WhiteNoise::new();

        let mut sum = 0.0;
        let n = 100_000;

        for _ in 0..n {
            let x = noise.process();
            assert!(x >= -1.0 && x <= 1.0, "noise out of range: {x}");
            sum += x;
        }

        let mean = sum / n as f32;
        assert!(mean.abs() < 0.02, "noise mean too far from zero: {mean}");
    }
}
