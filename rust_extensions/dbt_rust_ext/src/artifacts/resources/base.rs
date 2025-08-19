use crate::artifacts::resources::types::NodeType;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct BaseResource {
    pub name: String,
    pub resource_type: NodeType,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
}

impl BaseResource {
    pub fn new(
        name: String,
        resource_type: NodeType,
        package_name: String,
        path: String,
        original_file_path: String,
        unique_id: String,
    ) -> Self {
        Self {
            name,
            resource_type,
            package_name,
            path,
            original_file_path,
            unique_id,
        }
    }
}
