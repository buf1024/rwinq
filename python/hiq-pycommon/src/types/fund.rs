use pyo3::{PyObject, Python, IntoPy};
use pyo3::types::{PyDict, PyList};
use serde::{Serialize, Deserialize};
use crate::Bar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundInfo {
    pub(crate) inner: hiq_common::FundInfo,
}

impl From<hiq_common::FundInfo> for FundInfo {
    fn from(inner: hiq_common::FundInfo) -> Self {
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
    pub(crate) inner: hiq_common::FundNet,
}

impl From<hiq_common::FundNet> for FundNet {
    fn from(inner: hiq_common::FundNet) -> Self {
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
        dict.set_item("apply_status", self.inner.apply_status).unwrap();
        dict.set_item("redeem_status", self.inner.redeem_status).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundBar {
    pub(crate) inner: hiq_common::FundBar,
}

impl From<hiq_common::FundBar> for FundBar {
    fn from(inner: hiq_common::FundBar) -> Self {
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
            let new_bars: Vec<_> = bars.into_iter()
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

