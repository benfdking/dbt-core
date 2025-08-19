use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(module = "dbt_rust_ext", eq, eq_int, frozen)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParseFileType {
    #[pyo3(name = "Macro")]
    Macro,
    #[pyo3(name = "Model")]
    Model,
    #[pyo3(name = "Snapshot")]
    Snapshot,
    #[pyo3(name = "Analysis")]
    Analysis,
    #[pyo3(name = "SingularTest")]
    SingularTest,
    #[pyo3(name = "GenericTest")]
    GenericTest,
    #[pyo3(name = "Seed")]
    Seed,
    #[pyo3(name = "Documentation")]
    Documentation,
    #[pyo3(name = "Schema")]
    Schema,
    #[pyo3(name = "Hook")]
    Hook,
    #[pyo3(name = "Fixture")]
    Fixture,
}

#[pymethods]
impl ParseFileType {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        match s {
            "macro" => Ok(ParseFileType::Macro),
            "model" => Ok(ParseFileType::Model),
            "snapshot" => Ok(ParseFileType::Snapshot),
            "analysis" => Ok(ParseFileType::Analysis),
            "singular_test" => Ok(ParseFileType::SingularTest),
            "generic_test" => Ok(ParseFileType::GenericTest),
            "seed" => Ok(ParseFileType::Seed),
            "docs" => Ok(ParseFileType::Documentation),
            "schema" => Ok(ParseFileType::Schema),
            "hook" => Ok(ParseFileType::Hook),
            "fixture" => Ok(ParseFileType::Fixture),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown ParseFileType: {}",
                s
            ))),
        }
    }

    #[getter]
    pub fn value(&self) -> &'static str {
        match self {
            ParseFileType::Macro => "macro",
            ParseFileType::Model => "model",
            ParseFileType::Snapshot => "snapshot",
            ParseFileType::Analysis => "analysis",
            ParseFileType::SingularTest => "singular_test",
            ParseFileType::GenericTest => "generic_test",
            ParseFileType::Seed => "seed",
            ParseFileType::Documentation => "docs",
            ParseFileType::Schema => "schema",
            ParseFileType::Hook => "hook",
            ParseFileType::Fixture => "fixture",
        }
    }

    pub fn __str__(&self) -> &'static str {
        self.value()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ParseFileType.{}",
            match self {
                ParseFileType::Macro => "Macro",
                ParseFileType::Model => "Model",
                ParseFileType::Snapshot => "Snapshot",
                ParseFileType::Analysis => "Analysis",
                ParseFileType::SingularTest => "SingularTest",
                ParseFileType::GenericTest => "GenericTest",
                ParseFileType::Seed => "Seed",
                ParseFileType::Documentation => "Documentation",
                ParseFileType::Schema => "Schema",
                ParseFileType::Hook => "Hook",
                ParseFileType::Fixture => "Fixture",
            }
        )
    }

    pub fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[pyfunction]
pub fn parse_file_type_to_parser(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    let dict = PyDict::new_bound(py);
    dict.set_item("Macro", "MacroParser")?;
    dict.set_item("Model", "ModelParser")?;
    dict.set_item("Snapshot", "SnapshotParser")?;
    dict.set_item("Analysis", "AnalysisParser")?;
    dict.set_item("SingularTest", "SingularTestParser")?;
    dict.set_item("GenericTest", "GenericTestParser")?;
    dict.set_item("Seed", "SeedParser")?;
    dict.set_item("Documentation", "DocumentationParser")?;
    dict.set_item("Schema", "SchemaParser")?;
    dict.set_item("Hook", "HookParser")?;
    dict.set_item("Fixture", "FixtureParser")?;
    Ok(dict)
}

pub type PythonSourceFile = PyObject;

