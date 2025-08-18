use pyo3::prelude::*;

use crate::artifacts::resources::{base::BaseResource, v1::documentation::Documentation as DocumentationResource};

pub type Manifest = PyObject;


#[pyclass]
pub struct Documentation {
    base_resource: BaseResource,
    documentation: DocumentationResource
} 

impl Documentation {
    pub fn new(base_resource: BaseResource, documentation: DocumentationResource) -> Self {
        Self { base_resource, documentation }
    }
}