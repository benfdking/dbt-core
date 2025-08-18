use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct BaseParser {
    #[pyo3(get, set)]
    pub project: PyObject,
    #[pyo3(get, set)]
    pub manifest: PyObject,
}

#[pymethods]
impl BaseParser {
    #[new]
    pub fn new(project: PyObject, manifest: PyObject) -> Self {
        BaseParser { project, manifest }
    }

    #[pyo3(name = "parse_file")]
    fn parse_file(&self, _block: PyObject) -> PyResult<()> {
        // Abstract method: should be implemented by subclass
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "parse_file must be implemented by subclass",
        ))
    }

    #[getter]
    fn resource_type(&self) -> PyResult<PyObject> {
        // Abstract property: should be implemented by subclass
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "resource_type must be implemented by subclass",
        ))
    }
}
