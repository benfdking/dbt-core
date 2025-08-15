
use pyo3::prelude::*;
use pyo3::pyclass;

#[pyclass(module = "dbt_rust_ext", eq, eq_int, frozen)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AccessType {
    #[pyo3(name = "Private")]
    Private,
    #[pyo3(name = "Protected")]
    Protected,
    #[pyo3(name = "Public")]
    Public,
}

#[pymethods]
impl AccessType {
    #[staticmethod]
    pub fn is_valid(item: &str) -> bool {
        matches!(item, "private" | "protected" | "public")
    }

    #[getter]
    pub fn value(&self) -> &'static str {
        match self {
            AccessType::Private => "private",
            AccessType::Protected => "protected",
            AccessType::Public => "public",
        }
    }

    pub fn __str__(&self) -> &'static str {
        self.value()
    }

    pub fn __repr__(&self) -> String {
        format!("AccessType.{}", match self {
            AccessType::Private => "Private",
            AccessType::Protected => "Protected",
            AccessType::Public => "Public",
        })
    }

    pub fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[pyclass(module = "dbt_rust_ext", eq, eq_int, frozen)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NodeType {
    #[pyo3(name = "Model")]
    Model,
    #[pyo3(name = "Analysis")]
    Analysis,
    #[pyo3(name = "Test")]
    Test,
    #[pyo3(name = "Snapshot")]
    Snapshot,
    #[pyo3(name = "Operation")]
    Operation,
    #[pyo3(name = "Seed")]
    Seed,
    #[pyo3(name = "RPCCall")]
    RPCCall,
    #[pyo3(name = "SqlOperation")]
    SqlOperation,
    #[pyo3(name = "Documentation")]
    Documentation,
    #[pyo3(name = "Source")]
    Source,
    #[pyo3(name = "Macro")]
    Macro,
    #[pyo3(name = "Exposure")]
    Exposure,
    #[pyo3(name = "Metric")]
    Metric,
    #[pyo3(name = "Group")]
    Group,
    #[pyo3(name = "SavedQuery")]
    SavedQuery,
    #[pyo3(name = "SemanticModel")]
    SemanticModel,
    #[pyo3(name = "Unit")]
    Unit,
    #[pyo3(name = "Fixture")]
    Fixture,
}

#[pymethods]
impl NodeType {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        match s {
            "model" => Ok(NodeType::Model),
            "analysis" => Ok(NodeType::Analysis),
            "test" => Ok(NodeType::Test),
            "snapshot" => Ok(NodeType::Snapshot),
            "operation" => Ok(NodeType::Operation),
            "seed" => Ok(NodeType::Seed),
            "rpc" => Ok(NodeType::RPCCall),
            "sql_operation" => Ok(NodeType::SqlOperation),
            "doc" => Ok(NodeType::Documentation),
            "source" => Ok(NodeType::Source),
            "macro" => Ok(NodeType::Macro),
            "exposure" => Ok(NodeType::Exposure),
            "metric" => Ok(NodeType::Metric),
            "group" => Ok(NodeType::Group),
            "saved_query" => Ok(NodeType::SavedQuery),
            "semantic_model" => Ok(NodeType::SemanticModel),
            "unit_test" => Ok(NodeType::Unit),
            "fixture" => Ok(NodeType::Fixture),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unknown NodeType: {}", s))),
        }
    }

    pub fn pluralize(&self) -> String {
        match self {
            NodeType::Analysis => "analyses".to_string(),
            NodeType::SavedQuery => "saved_queries".to_string(),
            NodeType::Test => "data_tests".to_string(),
            _ => format!("{}s", self.as_str()),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Model => "model",
            NodeType::Analysis => "analysis",
            NodeType::Test => "test",
            NodeType::Snapshot => "snapshot",
            NodeType::Operation => "operation",
            NodeType::Seed => "seed",
            NodeType::RPCCall => "rpc",
            NodeType::SqlOperation => "sql_operation",
            NodeType::Documentation => "doc",
            NodeType::Source => "source",
            NodeType::Macro => "macro",
            NodeType::Exposure => "exposure",
            NodeType::Metric => "metric",
            NodeType::Group => "group",
            NodeType::SavedQuery => "saved_query",
            NodeType::SemanticModel => "semantic_model",
            NodeType::Unit => "unit_test",
            NodeType::Fixture => "fixture",
        }
    }

    #[getter]
    pub fn value(&self) -> &'static str {
        self.as_str()
    }

    pub fn __str__(&self) -> &'static str {
        self.as_str()
    }

    pub fn __repr__(&self) -> String {
        format!("NodeType.{}", match self {
            NodeType::Model => "Model",
            NodeType::Analysis => "Analysis",
            NodeType::Test => "Test",
            NodeType::Snapshot => "Snapshot",
            NodeType::Operation => "Operation",
            NodeType::Seed => "Seed",
            NodeType::RPCCall => "RPCCall",
            NodeType::SqlOperation => "SqlOperation",
            NodeType::Documentation => "Documentation",
            NodeType::Source => "Source",
            NodeType::Macro => "Macro",
            NodeType::Exposure => "Exposure",
            NodeType::Metric => "Metric",
            NodeType::Group => "Group",
            NodeType::SavedQuery => "SavedQuery",
            NodeType::SemanticModel => "SemanticModel",
            NodeType::Unit => "Unit",
            NodeType::Fixture => "Fixture",
        })
    }

    pub fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
