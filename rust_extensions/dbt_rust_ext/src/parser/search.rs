use pyo3::prelude::*;

use crate::{
    contracts::files::PythonSourceFile,
    dbt_common::clients::{jinja::extract_top_level_blocks, jinja_blocks::BlockTag},
};

pub type PythonFileBlock = PyAny;

pub fn python_file_block_get_file(block: &Bound<'_, PythonFileBlock>) -> PyResult<PythonSourceFile> {
    let file = block.getattr("file")?;
    Ok(file.into())
}

pub type BlockContents = PyAny;

pub fn block_contents_get_name(block_contents: &Bound<'_, BlockContents>) -> String {
    let name = block_contents.getattr("name").unwrap();
    name.extract::<String>().unwrap()
}

pub fn block_contents_get_contents(block_contents: &Bound<'_, BlockContents>) -> Option<String> {
    let contents = block_contents.getattr("contents").unwrap();
    contents.extract::<Option<String>>().unwrap()
}

// Extracts the relative path of the file from the block
// block.file.path.relative_path,
pub fn block_get_file_path_relative_path(block: &Bound<'_, PythonFileBlock>) -> String {
    let file = block.getattr("file").unwrap();
    let path = file.getattr("path").unwrap();
    let relative_path = path.getattr("relative_path").unwrap();
    relative_path.extract::<String>().unwrap()
}

// Extract block.path.original_file_path
pub fn block_get_file_path_original_file_path(block: &Bound<'_, PythonFileBlock>) -> String {
    let file = block.getattr("file").unwrap();
    let path = file.getattr("path").unwrap();
    let original_file_path = path.getattr("original_file_path").unwrap();
    original_file_path.extract::<String>().unwrap()
}

pub struct FileBlock {
    contents: String,
}

impl From<&Bound<'_, PythonFileBlock>> for FileBlock {
    fn from(block: &Bound<'_, PythonFileBlock>) -> Self {
        Self {
            contents: block
                .getattr("contents")
                .unwrap()
                .extract::<String>()
                .unwrap(),
        }
    }
}

pub struct BlockSearcher {
    source: Vec<FileBlock>,
    allowed_blocks: std::collections::HashSet<String>,
    check_jinja: bool,
}

impl BlockSearcher {
    pub fn new(
        source: Vec<FileBlock>,
        allowed_blocks: std::collections::HashSet<String>,
        check_jinja: Option<bool>,
    ) -> Self {
        Self {
            source,
            allowed_blocks,
            check_jinja: check_jinja.unwrap_or(true),
        }
    }

    pub fn extract_blocks(&self, block: &FileBlock) -> Vec<BlockTag> {
        let blocks =
            extract_top_level_blocks(&block.contents, Some(&self.allowed_blocks), false, None);
        blocks
    }

    pub fn iterator_return_all(&self) -> Vec<FileBlock> {
        let results: Vec<FileBlock> = Vec::new();
        unimplemented!()
        // for entry in self.source {
        //     for block in self.extract_blocks(&entry) {
        //         results.push(entry);
        //     }
        // }
        // results
    }
}
