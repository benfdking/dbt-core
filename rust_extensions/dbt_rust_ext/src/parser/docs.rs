use std::collections::HashSet;

use pyo3::{prelude::*, types::PyType};

use crate::{
    artifacts::{
        self,
        resources::{base::BaseResource, types::NodeType},
    },
    clients::jinja::get_rendered,
    config::runtime::RuntimeConfig,
    contracts::graph::{manifest_add_doc, Documentation, Manifest},
    parser::search::{
        block_contents_get_contents, block_contents_get_name,
        block_get_file_path_original_file_path, block_get_file_path_relative_path,
        python_file_block_get_file, BlockContents, BlockSearchResult, BlockSearcher, FileBlock,
        PythonFileBlock,
    },
};

#[pyclass]
pub struct DocumentationParser {
    project: RuntimeConfig,
    #[pyo3(get)]
    manifest: Manifest,
}

#[pymethods]
impl DocumentationParser {
    #[new]
    #[pyo3(signature = (project, root_project, manifest))]
    fn new(project: RuntimeConfig, root_project: RuntimeConfig, manifest: Manifest) -> Self {
        Self { project, manifest }
    }

    #[classmethod]
    fn get_compiled_path(
        cls: &Bound<'_, PyType>,
        block: &Bound<'_, PythonFileBlock>,
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
        Ok(self.generate_unique_id_block(project_name, resource_name.to_string()))
    }

    fn parse_block(
        &self,
        py: Python<'_>,
        block: &Bound<'_, PythonFileBlock>,
    ) -> Vec<Documentation> {
        let block: FileBlock = block.into();
        let project_name: String = self.project.project_name(py).unwrap();
        let block_contents = block_contents_get_contents(block).unwrap();
        let name = block_contents_get_name();
        let unique_id = self.generate_unique_id(py, &name, None).unwrap();
        let block_contents = block_contents_get_contents(block).unwrap();

        return self.parse_block_internal(
            project_name,
             unique_id,
             file_block_path_relative_path,
             block_contents_get_name(block),
             block_contents_get_contents(block).unwrap()
            );
    }

    fn parse_file(&self, py: Python<'_>, file_block: &Bound<'_, PythonFileBlock>) -> PyResult<()> {
        println!("Parsing file: {:?}", file_block);
        // Assert that file_block.file is a SourceFile
        let file = file_block.getattr("file")?;
        let source_file_type = file.get_type();
        if source_file_type.name()? != "SourceFile" {
            return Err(PyErr::new::<pyo3::exceptions::PyAssertionError, _>(
                "Expected file_block.file to be a SourceFile",
            ));
        }

        let block: FileBlock = file_block.into();

        let searcher = BlockSearcher::new(
            vec![block],
            HashSet::from_iter(vec!["docs".to_string()]),
            Box::new(BlockContents::factory()),
            Some(true),
        );
        let results = searcher.iterator_return_all();
        let blocks = results
            .into_iter()
            .map(|r| r.into())
            .collect::<Vec<FileBlock>>();
        println!("Blocks: {:?}", blocks);
        let project_name = self.project.project_name(py).unwrap();
        for block in blocks {
            let docs = self.parse_block_internal(project_name.clone(), &block.into());
            for doc in docs {
                let result = manifest_add_doc(
                    py,
                    &self.manifest,
                    python_file_block_get_file(file_block)?,
                    doc,
                );
                if result.is_err() {
                    panic!("Failed to add doc to manifest");
                }
            }
        }
        Ok(())
    }
}

impl From<Box<dyn BlockSearchResult>> for FileBlock {
    fn from(block: Box<dyn BlockSearchResult>) -> Self {
        let block_contents = block.contents();
        FileBlock {
            contents: block_contents,
        }
    }
}

impl DocumentationParser {
    fn generate_unique_id_block(&self, project_name: String, resource_name: String) -> String {
        format!("doc.{project_name}.{resource_name}")
    }

    fn parse_block_internal(
        &self,
        project_name: String,
        file_block_original_file_path: String,
        file_block_path_relative_path: String,
        file_block_conents_name: String,
        file_block_conents: String,
    ) -> Vec<Documentation> {
        let unique_id =
            self.generate_unique_id_block(project_name.clone(), file_block_conents_name.clone());

        let unstripped_conents = get_rendered(&file_block_conents, None);
        let stripped_contents = unstripped_conents.trim();

        let base_resource = BaseResource {
            path: file_block_path_relative_path,
            original_file_path: file_block_original_file_path,
            name: file_block_conents_name,
            resource_type: NodeType::Documentation,
            package_name: project_name,
            unique_id: unique_id,
        };
        let documentation_resource = artifacts::resources::v1::documentation::Documentation::new(
            base_resource.clone(),
            stripped_contents.to_string(),
        );
        let doc = Documentation::new(base_resource, documentation_resource);

        vec![doc]
    }
}
