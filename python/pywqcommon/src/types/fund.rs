use crate::Bar;
use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPy, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundInfo {
    pub(crate) inner: rwqcmm::FundInfo,
}

impl From<rwqcmm::FundInfo> for FundInfo {
    fn from(inner: rwqcmm::FundInfo) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for FundInfo {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundNet {
    pub(crate) inner: rwqcmm::FundNet,
}

impl From<rwqcmm::FundNet> for FundNet {
    fn from(inner: rwqcmm::FundNet) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for FundNet {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("trade_date", self.inner.trade_date).unwrap();
        dict.set_item("net", self.inner.net).unwrap();
        dict.set_item("net_acc", self.inner.net_acc).unwrap();
        dict.set_item("chg_pct", self.inner.chg_pct).unwrap();
        dict.set_item("apply_status", self.inner.apply_status)
            .unwrap();
        dict.set_item("redeem_status", self.inner.redeem_status)
            .unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundBar {
    pub(crate) inner: rwqcmm::FundBar,
}

impl From<rwqcmm::FundBar> for FundBar {
    fn from(inner: rwqcmm::FundBar) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for FundBar {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("freq", self.inner.freq as i32).unwrap();
        if let Some(bars) = self.inner.bars {
            let new_bars: Vec<_> = bars
                .into_iter()
                .map(|bar| Bar::from(bar).into_py(py))
                .collect();
            dict.set_item("bars", new_bars).unwrap();
        } else {
            let empty = PyList::new(py, Vec::<PyObject>::new());
            dict.set_item("bars", empty).unwrap();
        }
        dict.into()
    }
}
