pub mod foo;

use pyo3::prelude::*;

use crate::foo::sum_as_string;

#[pymodule]
fn rydia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
