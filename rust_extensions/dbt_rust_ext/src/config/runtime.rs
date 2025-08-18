use pyo3::prelude::*;
use pyo3::PyObject;


#[repr(transparent)]
pub struct RuntimeConfig(pub PyObject);

impl RuntimeConfig {
    pub fn new(inner: PyObject) -> Self {
        Self(inner)
    }

    pub fn project_name(&self, py: Python<'_>) -> PyResult<String> {
        let any = self.0.bind(py);
        any.getattr("project_name")?.extract()
    }

}

impl Clone for RuntimeConfig {
    fn clone(&self) -> Self {
        Python::with_gil(|py| RuntimeConfig(self.0.clone_ref(py)))
    }
}

impl<'py> FromPyObject<'py> for RuntimeConfig {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(RuntimeConfig(obj.clone().unbind()))
    }
}

