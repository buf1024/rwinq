use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StrategyType {
    pub(crate) inner: rwqstrategy::StrategyType,
}

impl From<rwqstrategy::StrategyType> for StrategyType {
    fn from(inner: rwqstrategy::StrategyType) -> Self {
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
    pub(crate) inner: rwqstrategy::Stat,
}

impl From<rwqstrategy::Stat> for Stat {
    fn from(inner: rwqstrategy::Stat) -> Self {
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
    pub(crate) inner: rwqstrategy::StrategyResult,
}

impl From<rwqstrategy::StrategyResult> for StrategyResult {
    fn from(inner: rwqstrategy::StrategyResult) -> Self {
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
