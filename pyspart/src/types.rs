use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper around PyObject to allow it to be used as a generic parameter in spart's data structures.
pub struct PyData(pub PyObject);

impl Clone for PyData {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyData(self.0.clone_ref(py)))
    }
}

impl PartialEq for PyData {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(
            |py| match self.0.bind(py).rich_compare(&other.0, CompareOp::Eq) {
                Ok(result) => result.is_truthy().unwrap_or(false),
                Err(_) => false,
            },
        )
    }
}

impl Eq for PyData {}

impl PartialOrd for PyData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Python::with_gil(|py| {
            let self_obj = self.0.bind(py);
            let other_obj = other.0.bind(py);
            if let Ok(result) = self_obj.rich_compare(other_obj, CompareOp::Lt) {
                if result.is_truthy().unwrap_or(false) {
                    return Some(std::cmp::Ordering::Less);
                }
            }
            if let Ok(result) = self_obj.rich_compare(other_obj, CompareOp::Gt) {
                if result.is_truthy().unwrap_or(false) {
                    return Some(std::cmp::Ordering::Greater);
                }
            }
            if let Ok(result) = self_obj.rich_compare(other_obj, CompareOp::Eq) {
                if result.is_truthy().unwrap_or(false) {
                    return Some(std::cmp::Ordering::Equal);
                }
            }
            None
        })
    }
}

impl std::fmt::Debug for PyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::with_gil(|py| {
            let repr = self
                .0
                .bind(py)
                .repr()
                .map(|r| r.to_string())
                .unwrap_or_else(|_| "<repr failed>".to_string());
            write!(f, "PyData({})", repr)
        })
    }
}

impl Serialize for PyData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Python::with_gil(|py| {
            let pickle = py.import("pickle").map_err(serde::ser::Error::custom)?;
            let bound_self = self.0.bind(py);
            let bytes = pickle
                .call_method1("dumps", (bound_self,))
                .map_err(serde::ser::Error::custom)?;
            let bytes: &[u8] = bytes.extract().map_err(serde::ser::Error::custom)?;
            serializer.serialize_bytes(bytes)
        })
    }
}

impl<'de> Deserialize<'de> for PyData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        Python::with_gil(|py| {
            let pickle = py.import("pickle").map_err(serde::de::Error::custom)?;
            let obj = pickle
                .call_method("loads", (PyBytes::new(py, &bytes),), None)
                .map_err(serde::de::Error::custom)?;
            Ok(PyData(obj.into()))
        })
    }
}
