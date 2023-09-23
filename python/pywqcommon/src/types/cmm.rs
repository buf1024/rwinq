use std::collections::HashMap;

use pyo3::types::PyDict;
use pyo3::{IntoPy, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub(crate) inner: rwqcmm::Bar,
}

impl From<rwqcmm::Bar> for Bar {
    fn from(inner: rwqcmm::Bar) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for Bar {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("trade_date", self.inner.trade_date).unwrap();
        dict.set_item("open", self.inner.open).unwrap();
        dict.set_item("close", self.inner.close).unwrap();
        dict.set_item("high", self.inner.high).unwrap();
        dict.set_item("low", self.inner.low).unwrap();
        dict.set_item("volume", self.inner.volume).unwrap();
        dict.set_item("amount", self.inner.amount).unwrap();
        dict.set_item("turnover", self.inner.turnover).unwrap();
        dict.set_item("chg_pct", self.inner.chg_pct).unwrap();
        dict.set_item("volume_chg_pct", self.inner.volume_chg_pct)
            .unwrap();
        dict.set_item("amount_chg_pct", self.inner.amount_chg_pct)
            .unwrap();
        dict.set_item("hfq_factor", self.inner.hfq_factor).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quot {
    pub(crate) inner: rwqcmm::Quot,
}

impl From<rwqcmm::Quot> for Quot {
    fn from(inner: rwqcmm::Quot) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for Quot {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("time", self.inner.time).unwrap();
        dict.set_item("last_close", self.inner.last_close).unwrap();
        dict.set_item("open", self.inner.open).unwrap();
        dict.set_item("high", self.inner.high).unwrap();
        dict.set_item("low", self.inner.low).unwrap();
        dict.set_item("now", self.inner.now).unwrap();
        dict.set_item("chg", self.inner.chg).unwrap();
        dict.set_item("chg_pct", self.inner.chg_pct).unwrap();
        dict.set_item("volume", self.inner.volume).unwrap();
        dict.set_item("amount", self.inner.amount).unwrap();
        dict.set_item("turnover", self.inner.turnover).unwrap();
        dict.set_item("total_value", self.inner.total_value)
            .unwrap();
        dict.set_item("currency_value", self.inner.currency_value)
            .unwrap();
        dict.set_item("is_trading", self.inner.is_trading).unwrap();
        dict.set_item("buy", self.inner.buy).unwrap();
        dict.set_item("sell", self.inner.sell).unwrap();
        dict.set_item("bid", self.inner.bid).unwrap();
        dict.set_item("ask", self.inner.ask).unwrap();
        dict.into()
    }
}

pub type RtQuot = HashMap<String, Quot>;
