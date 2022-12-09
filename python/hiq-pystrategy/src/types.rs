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
        let dict = PyDict::new(py);
        let list = PyList::new(py, self.inner.hit_chg_pct.into_iter());
        dict.set_item("hit_chg_pct", list).unwrap();
        dict.set_item("start", self.inner.start.into_py(py))
            .unwrap();
        dict.set_item("end", self.inner.end.into_py(py)).unwrap();
        dict.set_item("low", self.inner.low.into_py(py)).unwrap();
        dict.set_item("high", self.inner.high.into_py(py)).unwrap();
        dict.set_item("hit", self.inner.hit.into_py(py)).unwrap();
        dict.set_item("hit_max", self.inner.hit_max.into_py(py))
            .unwrap();
        dict.into()
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
        if let Some(m) = self.inner.mark {
            dict.set_item("mark", m.into_py(py)).unwrap();
        }
        if let Some(m) = self.inner.stat {
            let stat = Stat::from(m);
            dict.set_item("stat", stat.into_py(py)).unwrap();
        }
        dict.into()
    }
}
