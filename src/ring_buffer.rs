use pyo3::prelude::*;

fn ceil_to_pow2_usize(n: usize) -> usize {
    n.max(1).next_power_of_two()
}

fn do_linear_interp(x1: f32, x2: f32, t: f32) -> f32 {
    t * x2 + (1.0 - t) * x1
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct RingBufferS {
    #[pyo3(get)]
    max_delay_samp: usize,

    buf: Vec<f32>,
    write_index: usize,
    wrap_mask: usize,
}

#[pymethods]
impl RingBufferS {
    #[new]
    #[pyo3(signature = (max_delay_samp))]
    pub fn new(max_delay_samp: usize) -> Self {
        let buf_len = ceil_to_pow2_usize(max_delay_samp);
        let buf = vec![0.0_f32; buf_len];
        let wrap_mask = buf_len - 1;

        Self {
            max_delay_samp,
            buf,
            write_index: 0,
            wrap_mask,
        }
    }

    pub fn read(&self, delay_samp: usize) -> f32 {
        let read_index = (self.write_index.wrapping_sub(delay_samp)) & self.wrap_mask;
        self.buf[read_index]
    }

    pub fn write(&mut self, xn: f32) {
        self.buf[self.write_index] = xn;
        self.write_index = (self.write_index + 1) & self.wrap_mask;
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct RingBufferN {
    #[pyo3(get)]
    sample_rate: f32,

    #[pyo3(get)]
    max_delay_sec: f32,

    buf: Vec<f32>,
    write_index: usize,
    wrap_mask: usize,
}

#[pymethods]
impl RingBufferN {
    #[new]
    #[pyo3(signature = (sample_rate, max_delay_sec))]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        assert!(sample_rate > 0.0);
        assert!(max_delay_sec > 0.0);

        let max_delay_samp = (sample_rate * max_delay_sec).ceil() as usize;
        let buf_len = ceil_to_pow2_usize(max_delay_samp);
        let buf = vec![0.0_f32; buf_len];
        let wrap_mask = buf_len - 1;

        Self {
            sample_rate,
            max_delay_sec,
            buf,
            write_index: 0,
            wrap_mask,
        }
    }

    pub fn read(&self, delay_sec: f32) -> f32 {
        let delay_samp = (self.sample_rate * delay_sec).floor() as usize;
        let read_index = (self.write_index.wrapping_sub(delay_samp)) & self.wrap_mask;
        self.buf[read_index]
    }

    pub fn write(&mut self, xn: f32) {
        self.buf[self.write_index] = xn;
        self.write_index = (self.write_index + 1) & self.wrap_mask;
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct RingBufferL {
    #[pyo3(get)]
    sample_rate: f32,

    #[pyo3(get)]
    max_delay_sec: f32,

    buf: Vec<f32>,
    write_index: usize,
    wrap_mask: usize,
}

#[pymethods]
impl RingBufferL {
    #[new]
    #[pyo3(signature = (sample_rate, max_delay_sec))]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        assert!(sample_rate > 0.0);
        assert!(max_delay_sec > 0.0);

        let max_delay_samp = (sample_rate * max_delay_sec).ceil() as usize;
        let buf_len = ceil_to_pow2_usize(max_delay_samp);
        let buf = vec![0.0_f32; buf_len];
        let wrap_mask = buf_len - 1;

        Self {
            sample_rate,
            max_delay_sec,
            buf,
            write_index: 0,
            wrap_mask,
        }
    }

    pub fn read(&self, delay_sec: f32) -> f32 {
        let delay_samples_f = self.sample_rate * delay_sec;
        let delay_samp = delay_samples_f.floor() as usize;
        let frac = delay_samples_f - (delay_samp as f32);

        let read_index1 = (self.write_index.wrapping_sub(delay_samp)) & self.wrap_mask;
        let read_index2 = (read_index1.wrapping_sub(1)) & self.wrap_mask;

        let x1 = self.buf[read_index1];
        let x2 = self.buf[read_index2];

        do_linear_interp(x1, x2, frac)
    }

    pub fn write(&mut self, xn: f32) {
        self.buf[self.write_index] = xn;
        self.write_index = (self.write_index + 1) & self.wrap_mask;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, eps: f32) {
        assert!(
            (a - b).abs() <= eps,
            "not approx equal: a={a}, b={b}, |a-b|={}",
            (a - b).abs()
        );
    }

    #[test]
    fn test_ceil_to_pow2_usize() {
        assert_eq!(ceil_to_pow2_usize(0), 1);
        assert_eq!(ceil_to_pow2_usize(1), 1);
        assert_eq!(ceil_to_pow2_usize(2), 2);
        assert_eq!(ceil_to_pow2_usize(3), 4);
        assert_eq!(ceil_to_pow2_usize(4), 4);
        assert_eq!(ceil_to_pow2_usize(5), 8);
        assert_eq!(ceil_to_pow2_usize(8), 8);
        assert_eq!(ceil_to_pow2_usize(9), 16);
    }

    #[test]
    fn test_do_linear_interp() {
        approx_eq(do_linear_interp(1.0, 2.0, 0.0), 1.0, 1e-7);
        approx_eq(do_linear_interp(1.0, 2.0, 0.1), 1.1, 1e-7);
        approx_eq(do_linear_interp(1.0, 2.0, 0.9), 1.9, 1e-7);
        approx_eq(do_linear_interp(1.0, 2.0, 1.0), 2.0, 1e-7);
    }

    #[test]
    fn test_ring_buffer_s_reads_expected_history() {
        let mut rb = RingBufferS::new(3);
        for i in 10..=15 {
            rb.write(i as f32);
        }

        // 直近の履歴を読む（内部バッファ配置には依存しない）
        approx_eq(rb.read(1), 15.0, 1e-6);
        approx_eq(rb.read(2), 14.0, 1e-6);
        approx_eq(rb.read(3), 13.0, 1e-6);
        approx_eq(rb.read(4), 12.0, 1e-6);
    }

    #[test]
    fn test_ring_buffer_n_truncates_fractional_delay() {
        let mut rb = RingBufferN::new(1.0, 3.0);
        for i in 10..=15 {
            rb.write(i as f32);
        }

        // delay_sec は floor でサンプルに落とす
        approx_eq(rb.read(1.0), 15.0, 1e-6);
        approx_eq(rb.read(1.1), 15.0, 1e-6);
        approx_eq(rb.read(1.9), 15.0, 1e-6);
        approx_eq(rb.read(2.0), 14.0, 1e-6);
        approx_eq(rb.read(3.0), 13.0, 1e-6);
        approx_eq(rb.read(4.0), 12.0, 1e-6);
    }

    #[test]
    fn test_ring_buffer_l_linearly_interpolates() {
        let mut rb = RingBufferL::new(1.0, 3.0);
        for i in 10..=15 {
            rb.write(i as f32);
        }

        approx_eq(rb.read(1.0), 15.0, 1e-6);
        approx_eq(rb.read(1.1), 14.9, 1e-4);
        approx_eq(rb.read(1.9), 14.1, 1e-4);
        approx_eq(rb.read(2.0), 14.0, 1e-6);
        approx_eq(rb.read(3.0), 13.0, 1e-6);
        approx_eq(rb.read(4.0), 12.0, 1e-6);
    }
}
