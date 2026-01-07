//! rydia-core Python module entry point
//!
//! Note:
//! - Type stubs are generated for internal development convenience only.
//! - Stub files are NOT part of the public API in v0.1.0.

pub mod foo;

use pyo3::prelude::*;

// --- Stub generation (internal use only) -------------------------------
//
// This is intentionally kept separate from the pymodule definition.
// In v0.1.0, generated .pyi files are NOT distributed and are ignored
// via .gitignore.
//
use pyo3_stub_gen::define_stub_info_gatherer;
define_stub_info_gatherer!(stub_info);

// --- Python-exposed API -------------------------------------------------

use crate::foo::sum_as_string;

#[pymodule]
fn rydia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
