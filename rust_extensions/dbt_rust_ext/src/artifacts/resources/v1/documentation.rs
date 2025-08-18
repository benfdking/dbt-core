use pyo3::prelude::*;

use crate::artifacts::resources::base::BaseResource;

#[pyclass]
pub struct Documentation {
    base_resource: BaseResource,
    block_contents: String,
}

impl Documentation {
    fn new(base_resource: BaseResource, block_contents: String) -> Self {
        Self {
            base_resource,
            block_contents,
        }
    }
}
