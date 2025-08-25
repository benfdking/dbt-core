use pyo3::prelude::*;

use crate::{
    artifacts::resources::{
        base::BaseResource, types::NodeType,
        v1::documentation::Documentation as DocumentationResource,
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

#[pymethods]
impl Documentation {
    #[getter]
    fn unique_id(&self) -> String {
        get_unique_id(
            NodeType::Documentation,
            self.base_resource.package_name.clone(),
            self.base_resource.name.clone(),
            None,
        )
    }

    #[getter]
    fn name(&self) -> String {
        self.base_resource.name.clone()
    }

    #[getter]
    fn resource_type<'a>(&self, py: Python<'a>) -> pyo3::Bound<'a, pyo3::PyAny> {
        let builtins = PyModule::import_bound(py, "dbt.artifacts.resources.types").unwrap();
        let node_type = builtins.getattr("NodeType").unwrap();
        let documentation = node_type.getattr("Documentation");
        documentation.unwrap()
    }

    #[getter]
    fn original_file_path(&self) -> String {
        self.base_resource.original_file_path.clone()
    }

    #[getter]
    fn package_name(&self) -> String {
        self.base_resource.package_name.clone()
    }

    #[getter]
    fn path(&self) -> String {
        self.base_resource.path.clone()
    }
}

fn get_unique_id(
    node_type: NodeType,
    package_name: String,
    resource_name: String,
    version: Option<String>,
) -> String {
    let mut unique_id = format!("{}.{}.{}", node_type.value(), package_name, resource_name);
    if let Some(version) = version {
        unique_id = format!("{}.v{}", unique_id, version);
    }
    unique_id
}
