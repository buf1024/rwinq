use pyo3::types::PyDict;
use pyo3::{IntoPy, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub(crate) inner: hiq_common::Bar,
}

impl From<hiq_common::Bar> for Bar {
    fn from(inner: hiq_common::Bar) -> Self {
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
        dict.set_item("hfq_factor", self.inner.hfq_factor).unwrap();
        dict.into()
    }
}
