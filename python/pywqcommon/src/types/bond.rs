use crate::Bar;
use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPy, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondInfo {
    pub(crate) inner: rwqcmm::BondInfo,
}

impl From<rwqcmm::BondInfo> for BondInfo {
    fn from(inner: rwqcmm::BondInfo) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for BondInfo {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("stock_code", self.inner.stock_code).unwrap();
        dict.set_item("stock_name", self.inner.stock_name).unwrap();
        dict.set_item("listing_date", self.inner.listing_date)
            .unwrap();
        dict.set_item("is_delist", self.inner.is_delist).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondBar {
    pub(crate) inner: rwqcmm::BondBar,
}

impl From<rwqcmm::BondBar> for BondBar {
    fn from(inner: rwqcmm::BondBar) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for BondBar {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("stock_code", self.inner.stock_code).unwrap();
        dict.set_item("stock_name", self.inner.stock_name).unwrap();
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
