use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StrategyType {
    pub(crate) inner: hiq_strategy::StrategyType,
}

impl From<hiq_strategy::StrategyType> for StrategyType {
    fn from(inner: hiq_strategy::StrategyType) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StrategyType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.inner as i32).to_object(py)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub(crate) inner: hiq_strategy::Marker,
}

impl From<hiq_strategy::Marker> for Marker {
    fn from(inner: hiq_strategy::Marker) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for Marker {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("key", self.inner.key).unwrap();
        dict.set_item("val", self.inner.val).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub(crate) inner: hiq_strategy::Stat,
}

impl From<hiq_strategy::Stat> for Stat {
    fn from(inner: hiq_strategy::Stat) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for Stat {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let list = PyList::new(py, self.inner.chg_pct.into_iter());

        list.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyResult {
    pub(crate) inner: hiq_strategy::StrategyResult,
}

impl From<hiq_strategy::StrategyResult> for StrategyResult {
    fn from(inner: hiq_strategy::StrategyResult) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StrategyResult {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        if let Some(m) = self.inner.marker {
            let marker: Vec<_> = m
                .into_iter()
                .map(|e| {
                    let mk = Marker::from(e);
                    mk.into_py(py)
                })
                .collect();
            dict.set_item("marker", marker).unwrap();
        }
        if let Some(m) = self.inner.stat {
            let stat = Stat::from(m).into_py(py);
            dict.set_item("stat", stat).unwrap();
        }
        dict.into()
    }
}
