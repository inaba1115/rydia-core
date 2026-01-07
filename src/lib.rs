pub mod lfo;
pub mod osc;

use pyo3::prelude::*;

use crate::lfo::Lfo;
use crate::osc::{SinOsc, WhiteNoise};

#[pymodule]
fn rydia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Lfo>()?;

    m.add_class::<SinOsc>()?;
    m.add_class::<WhiteNoise>()?;

    Ok(())
}
