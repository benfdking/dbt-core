use pyo3::{prelude::*, types::PyType};

use crate::{
    artifacts::resources::{base::BaseResource, types::NodeType},
    clients::jinja::get_rendered,
    config::runtime::RuntimeConfig,
    contracts::graph::{Documentation, Manifest},
    parser::search::{block_contents_get_contents, block_contents_get_name, FileBlock},
};

#[pyclass]
pub struct DocumentationParser {
    project: RuntimeConfig,
}

#[pymethods]
impl DocumentationParser {
    #[new]
    #[pyo3(signature = (project, root_project, manifest))]
    fn new(project: RuntimeConfig, root_project: RuntimeConfig, manifest: Manifest) -> Self {
        Self { project }
    }

    #[classmethod]
    fn get_compiled_path(
        cls: &Bound<'_, PyType>,
        block: &Bound<'_, FileBlock>,
    ) -> PyResult<PyObject> {
        // Assumes block has attribute 'path', which has attribute 'relative_path'
        let path = block.getattr("path")?;
        let relative_path = path.getattr("relative_path")?;
        Ok(relative_path.into())
    }

    fn resource_type(&self) -> NodeType {
        NodeType::Documentation
    }

    // def generate_unique_id(self, resource_name: str, _: Optional[str] = None) -> str:
    // # For consistency, use the same format for doc unique_ids
    // return f"doc.{self.project.project_name}.{resource_name}"

    fn generate_unique_id(
        &self,
        py: Python<'_>,
        resource_name: &str,
        _ignored: Option<&str>,
    ) -> PyResult<String> {
        let project_name = self.project.project_name(py)?; // needs GIL + returns PyResult
        Ok(format!("doc.{project_name}.{resource_name}"))
    }

    fn parse_block(&self, py: Python<'_>, block: &Bound<'_, FileBlock>) -> Vec<Documentation> {
        let name = block_contents_get_name(block);
        let unique_id = self.generate_unique_id(py, &name, None).unwrap();

        let block_contents = block_contents_get_contents(block).unwrap();
        let unstripped_conents = get_rendered(&block_contents, None);
        let stripped_contents = unstripped_conents.trim();

        let base_resource = BaseResource::new(

        );
        let doc = Documentation::new(base_resource, stripped_contents);

        unimplemented!()
    }

    fn parse_file(&self, file_block: &Bound<'_, FileBlock>) -> PyResult<Vec<Documentation>> {
        panic!("Not implemented");
    }
}
