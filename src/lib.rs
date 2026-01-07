pub mod foo;
pub mod osc;

use pyo3::prelude::*;

use crate::foo::sum_as_string;
use crate::osc::{SinOsc, WhiteNoise};

#[pymodule]
fn rydia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add_class::<SinOsc>()?;
    m.add_class::<WhiteNoise>()?;

    Ok(())
}
