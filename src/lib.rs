pub mod lfo;
pub mod osc;
pub mod ring_buffer;

use pyo3::prelude::*;

use crate::lfo::Lfo;
use crate::osc::{SinOsc, WhiteNoise};
use crate::ring_buffer::{RingBufferL, RingBufferN, RingBufferS};

#[pymodule]
fn rydia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Lfo>()?;

    m.add_class::<SinOsc>()?;
    m.add_class::<WhiteNoise>()?;

    m.add_class::<RingBufferS>()?;
    m.add_class::<RingBufferN>()?;
    m.add_class::<RingBufferL>()?;

    Ok(())
}
