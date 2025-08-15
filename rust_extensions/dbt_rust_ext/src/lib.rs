pub mod resources;
pub mod parser;
pub mod contracts;

use pyo3::prelude::*;
use std::path::PathBuf;
use parser::partial::key_to_prefix;
use resources::types::{AccessType, NodeType};
use contracts::files::{ParseFileType, parse_file_type_to_parser};

/// Get the path to the profiles.yml file
/// This is a Rust implementation of the Python get_profiles_path function
#[pyfunction]
fn get_profiles_path(profiles_dir: &str) -> PyResult<String> {
    let mut path = PathBuf::from(profiles_dir);
    path.push("profiles.yml");
    
    Ok(path.to_string_lossy().into_owned())
}

/// A Python module implemented in Rust using PyO3
#[pymodule]
fn dbt_rust_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_profiles_path, m)?)?;
    m.add_function(wrap_pyfunction!(key_to_prefix, m)?)?;
    m.add_function(wrap_pyfunction!(parse_file_type_to_parser, m)?)?;
    m.add_class::<AccessType>()?;
    m.add_class::<NodeType>()?;
    m.add_class::<ParseFileType>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_profiles_path() {
        let result = get_profiles_path("/home/user/.dbt").unwrap();
        assert_eq!(result, "/home/user/.dbt/profiles.yml");
    }

    #[test]
    fn test_get_profiles_path_windows_style() {
        let result = get_profiles_path("C:\\Users\\user\\.dbt").unwrap();
        assert_eq!(result, "C:\\Users\\user\\.dbt\\profiles.yml");
    }
}