pub mod delay;
pub mod iir_filter;
pub mod lfo;
pub mod osc;
pub mod ring_buffer;

use pyo3::prelude::*;

use crate::delay::{
    AllpassL, AllpassN, AllpassS, CombL, CombN, CombS, DelayL, DelayN, DelayS, calc_fb,
};
use crate::iir_filter::Biquad;
use crate::lfo::Lfo;
use crate::osc::{SinOsc, WhiteNoise};
use crate::ring_buffer::{RingBufferL, RingBufferN, RingBufferS};

#[pymodule]
fn rydia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_fb, m)?)?;
    m.add_class::<DelayS>()?;
    m.add_class::<DelayN>()?;
    m.add_class::<DelayL>()?;
    m.add_class::<CombS>()?;
    m.add_class::<CombN>()?;
    m.add_class::<CombL>()?;
    m.add_class::<AllpassS>()?;
    m.add_class::<AllpassN>()?;
    m.add_class::<AllpassL>()?;

    m.add_class::<Biquad>()?;

    m.add_class::<Lfo>()?;

    m.add_class::<SinOsc>()?;
    m.add_class::<WhiteNoise>()?;

    m.add_class::<RingBufferS>()?;
    m.add_class::<RingBufferN>()?;
    m.add_class::<RingBufferL>()?;

    Ok(())
}
