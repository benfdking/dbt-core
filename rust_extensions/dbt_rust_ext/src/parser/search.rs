use pyo3::prelude::*;

pub type FileBlock = PyAny;

pub type BlockContents = PyAny;

pub fn block_contents_get_name(block_contents: &Bound<'_, BlockContents>) -> String {
    let name = block_contents.getattr("name").unwrap();
    name.extract::<String>().unwrap()
}

pub fn block_contents_get_contents(block_contents: &Bound<'_, BlockContents>) -> Option<String> {
    let contents = block_contents.getattr("contents").unwrap();
    contents.extract::<Option<String>>().unwrap()
}
