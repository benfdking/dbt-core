use pyo3::prelude::*;

use crate::{
    artifacts::resources::{
        base::BaseResource, v1::documentation::Documentation as DocumentationResource,
    },
    contracts::files::PythonSourceFile,
};

pub type Manifest = PyObject;

pub fn manifest_add_doc(
    python: Python<'_>,
    manifest: &Manifest,
    source_file: PythonSourceFile,
    doc: Documentation,
) -> PyResult<()> {
    let manifest_obj = manifest.bind(python);
    manifest_obj.call_method1("add_doc", (source_file, doc))?;
    Ok(())
}

#[pyclass]
pub struct Documentation {
    base_resource: BaseResource,
    documentation: DocumentationResource,
}

impl Documentation {
    pub fn new(base_resource: BaseResource, documentation: DocumentationResource) -> Self {
        Self {
            base_resource,
            documentation,
        }
    }
}
