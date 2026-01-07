use pyo3::prelude::*;

fn ceil_to_pow2(samp: f32) -> usize {
    2usize.pow((samp as f32).log2().ceil() as u32)
}

fn do_liner_interp(x1: f32, x2: f32, fractional_x: f32) -> f32 {
    fractional_x * x2 + (1.0 - fractional_x) * x1
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct RingBufferS {
    #[pyo3(get, set)]
    pub max_delay_samp: usize,

    #[pyo3(get, set)]
    pub buf: Vec<f32>,

    #[pyo3(get, set)]
    pub write_index: isize,

    #[pyo3(get, set)]
    pub wrap_mask: isize,
}

#[pymethods]
impl RingBufferS {
    #[new]
    #[pyo3(signature = (max_delay_samp))]
    pub fn new(max_delay_samp: usize) -> Self {
        let buf_len = ceil_to_pow2(max_delay_samp as f32);
        let buf = vec![0.0_f32; buf_len];
        let wrap_mask = (buf_len - 1) as isize;

        RingBufferS {
            max_delay_samp: max_delay_samp,
            buf: buf,
            write_index: 0,
            wrap_mask: wrap_mask,
        }
    }

    pub fn read(&self, delay_samp: usize) -> f32 {
        let read_index = self.write_index - delay_samp as isize;
        self.buf[(read_index & self.wrap_mask) as usize]
    }

    pub fn write(&mut self, xn: f32) {
        self.buf[self.write_index as usize] = xn;
        self.write_index += 1;
        self.write_index &= self.wrap_mask;
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct RingBufferN {
    #[pyo3(get, set)]
    pub sample_rate: f32,

    #[pyo3(get, set)]
    pub max_delay_sec: f32,

    #[pyo3(get, set)]
    pub buf: Vec<f32>,

    #[pyo3(get, set)]
    pub write_index: isize,

    #[pyo3(get, set)]
    pub wrap_mask: isize,
}

#[pymethods]
impl RingBufferN {
    #[new]
    #[pyo3(signature = (sample_rate, max_delay_sec))]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        let buf_len = ceil_to_pow2(sample_rate * max_delay_sec);
        let buf = vec![0.0_f32; buf_len];
        let wrap_mask = (buf_len - 1) as isize;

        RingBufferN {
            sample_rate: sample_rate,
            max_delay_sec: max_delay_sec,
            buf: buf,
            write_index: 0,
            wrap_mask: wrap_mask,
        }
    }

    pub fn read(&self, delay_sec: f32) -> f32 {
        let delay_samp = (self.sample_rate * delay_sec) as isize;
        let read_index = self.write_index - delay_samp;
        self.buf[(read_index & self.wrap_mask) as usize]
    }

    pub fn write(&mut self, xn: f32) {
        self.buf[self.write_index as usize] = xn;
        self.write_index += 1;
        self.write_index &= self.wrap_mask;
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct RingBufferL {
    #[pyo3(get, set)]
    pub sample_rate: f32,

    #[pyo3(get, set)]
    pub max_delay_sec: f32,

    #[pyo3(get, set)]
    pub buf: Vec<f32>,

    #[pyo3(get, set)]
    pub write_index: isize,

    #[pyo3(get, set)]
    pub wrap_mask: isize,
}

#[pymethods]
impl RingBufferL {
    #[new]
    #[pyo3(signature = (sample_rate, max_delay_sec))]
    pub fn new(sample_rate: f32, max_delay_sec: f32) -> Self {
        let buf_len = ceil_to_pow2(sample_rate * max_delay_sec);
        let buf = vec![0.0_f32; buf_len];
        let wrap_mask = (buf_len - 1) as isize;

        RingBufferL {
            sample_rate: sample_rate,
            max_delay_sec: max_delay_sec,
            buf: buf,
            write_index: 0,
            wrap_mask: wrap_mask,
        }
    }

    pub fn read(&self, delay_sec: f32) -> f32 {
        let delay_samp = (self.sample_rate * delay_sec) as isize;
        let read_index1 = self.write_index - delay_samp;
        let read_index2 = read_index1 - 1;
        let x1 = self.buf[(read_index1 & self.wrap_mask) as usize];
        let x2 = self.buf[(read_index2 & self.wrap_mask) as usize];
        let fractional_x = self.sample_rate * delay_sec - (delay_samp as f32);
        do_liner_interp(x1, x2, fractional_x)
    }

    pub fn write(&mut self, xn: f32) {
        self.buf[self.write_index as usize] = xn;
        self.write_index += 1;
        self.write_index &= self.wrap_mask;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceil_to_pow2() {
        assert_eq!(ceil_to_pow2(1.0), 1);
        assert_eq!(ceil_to_pow2(1.1), 2);
        assert_eq!(ceil_to_pow2(2.0), 2);
        assert_eq!(ceil_to_pow2(2.1), 4);
        assert_eq!(ceil_to_pow2(4.0), 4);
        assert_eq!(ceil_to_pow2(4.1), 8);
        assert_eq!(ceil_to_pow2(8.1), 16);
    }

    #[test]
    fn test_do_liner_interp() {
        assert_eq!(do_liner_interp(1.0, 2.0, 0.0), 1.0);
        assert_eq!(do_liner_interp(1.0, 2.0, 0.1), 1.1);
        assert_eq!(do_liner_interp(1.0, 2.0, 0.9), 1.9);
        assert_eq!(do_liner_interp(1.0, 2.0, 1.0), 2.0);
    }

    #[test]
    fn test_ring_buffer_s() {
        let mut rb = RingBufferS::new(3);
        for i in 10..=15 {
            rb.write(i as f32);
        }
        assert_eq!(rb.buf, vec![14.0, 15.0, 12.0, 13.0]);
        assert_eq!(rb.read(1), 15.0);
        assert_eq!(rb.read(2), 14.0);
        assert_eq!(rb.read(3), 13.0);
        assert_eq!(rb.read(4), 12.0);
    }

    #[test]
    fn test_ring_buffer_n() {
        let mut rb = RingBufferN::new(1.0, 3.0);
        for i in 10..=15 {
            rb.write(i as f32);
        }
        assert_eq!(rb.buf, vec![14.0, 15.0, 12.0, 13.0]);
        assert_eq!(rb.read(1.0), 15.0);
        assert_eq!(rb.read(1.1), 15.0);
        assert_eq!(rb.read(1.9), 15.0);
        assert_eq!(rb.read(2.0), 14.0);
        assert_eq!(rb.read(2.0), 14.0);
        assert_eq!(rb.read(3.0), 13.0);
        assert_eq!(rb.read(4.0), 12.0);
    }

    #[test]
    fn test_ring_buffer_l() {
        let mut rb = RingBufferL::new(1.0, 3.0);
        for i in 10..=15 {
            rb.write(i as f32);
        }
        assert_eq!(rb.buf, vec![14.0, 15.0, 12.0, 13.0]);
        assert_eq!(rb.read(1.0), 15.0);
        assert_eq!(rb.read(1.1), 14.900001);
        assert_eq!(rb.read(1.9), 14.099999);
        assert_eq!(rb.read(2.0), 14.0);
        assert_eq!(rb.read(2.0), 14.0);
        assert_eq!(rb.read(3.0), 13.0);
        assert_eq!(rb.read(4.0), 12.0);
    }
}
