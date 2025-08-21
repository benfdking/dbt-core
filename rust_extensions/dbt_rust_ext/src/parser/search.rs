use pyo3::prelude::*;

use dbt_common::clients::{jinja::extract_top_level_blocks, jinja_blocks::BlockTag};

use crate::contracts::files::PythonSourceFile;

pub type PythonFileBlock = PyAny;

pub fn python_file_block_get_file(
    block: &Bound<'_, PythonFileBlock>,
) -> PyResult<PythonSourceFile> {
    let file = block.getattr("file")?;
    Ok(file.into())
}

pub type PythonBlockContents = PyAny;

pub fn block_contents_get_name(block_contents: &Bound<'_, PythonBlockContents>) -> String {
    let name = block_contents.getattr("name").unwrap();
    name.extract::<String>().unwrap()
}

pub fn block_contents_get_contents(
    block_contents: &Bound<'_, PythonBlockContents>,
) -> Option<String> {
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

#[derive(Debug, Clone)]
pub struct FileBlock {
    pub contents: String,
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

#[pyclass]
#[derive(Debug, Clone)]
pub struct BlockContents {
    #[pyo3(get)]
    contents: String,
}

impl BlockContents {
    pub fn factory() -> SourceTagFactory {
        Box::new(|block_tag| {
            println!("Block tag: {:?}", block_tag);
            Box::new(BlockContents {
                contents: block_tag.contents.unwrap(),
            })
        })
    }
}

pub trait BlockSearchResult {
    fn contents(&self) -> String;
}

impl BlockSearchResult for BlockContents {
    fn contents(&self) -> String {
        self.contents.clone()
    }
}

pub struct BlockSearcher {
    source: Vec<FileBlock>,
    allowed_blocks: std::collections::HashSet<String>,
    check_jinja: bool,
    source_tag_factory: SourceTagFactory,
}

type SourceTagFactory = Box<dyn Fn(BlockTag) -> Box<dyn BlockSearchResult>>;

impl BlockSearcher {
    pub fn new(
        source: Vec<FileBlock>,
        allowed_blocks: std::collections::HashSet<String>,
        source_tag_factory: SourceTagFactory,
        check_jinja: Option<bool>,
    ) -> Self {
        Self {
            source,
            allowed_blocks,
            check_jinja: check_jinja.unwrap_or(true),
            source_tag_factory,
        }
    }

    pub fn extract_blocks(&self, block: &FileBlock) -> Vec<BlockTag> {
        let blocks =
            extract_top_level_blocks(&block.contents, Some(&self.allowed_blocks), false, None);
        blocks
    }

    pub fn iterator_return_all(&self) -> Vec<Box<dyn BlockSearchResult>> {
        let mut results: Vec<Box<dyn BlockSearchResult>> = Vec::new();
        for entry in &self.source {
            for block in self.extract_blocks(&entry) {
                let out = (self.source_tag_factory)(block);
                results.push(out);
            }
        }
        results
    }
}
