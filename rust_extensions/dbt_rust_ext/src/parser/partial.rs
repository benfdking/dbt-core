use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Expose the key_to_prefix constant as a Python dictionary.
/// This function returns a new dict each time, as Rust does not support exporting
/// Python-level constants directly, but this is the idiomatic way to expose such data.
#[pyfunction]
pub fn key_to_prefix(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    let dict = PyDict::new_bound(py);
    dict.set_item("models", "model")?;
    dict.set_item("seeds", "seed")?;
    dict.set_item("snapshots", "snapshot")?;
    dict.set_item("analyses", "analysis")?;
    Ok(dict)
}
