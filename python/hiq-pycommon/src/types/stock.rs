use crate::Bar;
use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPy, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    pub(crate) inner: hiq_common::StockInfo,
}

impl From<hiq_common::StockInfo> for StockInfo {
    fn from(inner: hiq_common::StockInfo) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockInfo {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("block", self.inner.block).unwrap();
        dict.set_item("is_margin", self.inner.is_margin).unwrap();
        dict.set_item("listing_date", self.inner.listing_date)
            .unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockBar {
    pub(crate) inner: hiq_common::StockBar,
}

impl From<hiq_common::StockBar> for StockBar {
    fn from(inner: hiq_common::StockBar) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockBar {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndex {
    pub(crate) inner: hiq_common::StockIndex,
}

impl From<hiq_common::StockIndex> for StockIndex {
    fn from(inner: hiq_common::StockIndex) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockIndex {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("trade_date", self.inner.trade_date).unwrap();
        dict.set_item("price", self.inner.price).unwrap();
        dict.set_item("pe", self.inner.pe).unwrap();
        dict.set_item("pb", self.inner.pb).unwrap();
        dict.set_item("total_value", self.inner.total_value)
            .unwrap();
        dict.set_item("currency_value", self.inner.currency_value)
            .unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndustry {
    pub(crate) inner: hiq_common::StockIndustry,
}

impl From<hiq_common::StockIndustry> for StockIndustry {
    fn from(inner: hiq_common::StockIndustry) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockIndustry {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndustryDetail {
    pub(crate) inner: hiq_common::StockIndustryDetail,
}

impl From<hiq_common::StockIndustryDetail> for StockIndustryDetail {
    fn from(inner: hiq_common::StockIndustryDetail) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockIndustryDetail {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("stock_code", self.inner.stock_code).unwrap();
        dict.set_item("stock_name", self.inner.stock_name).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndustryBar {
    pub(crate) inner: hiq_common::StockIndustryBar,
}

impl From<hiq_common::StockIndustryBar> for StockIndustryBar {
    fn from(inner: hiq_common::StockIndustryBar) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockIndustryBar {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockConcept {
    pub(crate) inner: hiq_common::StockConcept,
}

impl From<hiq_common::StockConcept> for StockConcept {
    fn from(inner: hiq_common::StockConcept) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockConcept {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockConceptDetail {
    pub(crate) inner: hiq_common::StockConceptDetail,
}

impl From<hiq_common::StockConceptDetail> for StockConceptDetail {
    fn from(inner: hiq_common::StockConceptDetail) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockConceptDetail {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("stock_code", self.inner.stock_code).unwrap();
        dict.set_item("stock_name", self.inner.stock_name).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockConceptBar {
    pub(crate) inner: hiq_common::StockConceptBar,
}

impl From<hiq_common::StockConceptBar> for StockConceptBar {
    fn from(inner: hiq_common::StockConceptBar) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockConceptBar {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockYJBB {
    pub(crate) inner: hiq_common::StockYJBB,
}

impl From<hiq_common::StockYJBB> for StockYJBB {
    fn from(inner: hiq_common::StockYJBB) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockYJBB {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("year", self.inner.year).unwrap();
        dict.set_item("season", self.inner.season).unwrap();
        dict.set_item("season_date", self.inner.season_date)
            .unwrap();
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("mg_sy", self.inner.mg_sy).unwrap();
        dict.set_item("yysr", self.inner.yysr).unwrap();
        dict.set_item("yysr_tbzz", self.inner.yysr_tbzz).unwrap();
        dict.set_item("yysr_jdhbzz", self.inner.yysr_jdhbzz)
            .unwrap();
        dict.set_item("jlr", self.inner.jlr).unwrap();
        dict.set_item("jlr_tbzz", self.inner.jlr_tbzz).unwrap();
        dict.set_item("jlr_jdhbzz", self.inner.jlr_jdhbzz).unwrap();
        dict.set_item("mg_jzc", self.inner.mg_jzc).unwrap();
        dict.set_item("jzc_syl", self.inner.jzc_syl).unwrap();
        dict.set_item("mg_jy_xjl", self.inner.mg_jy_xjl).unwrap();
        dict.set_item("xs_mll", self.inner.xs_mll).unwrap();
        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMargin {
    pub(crate) inner: hiq_common::StockMargin,
}

impl From<hiq_common::StockMargin> for StockMargin {
    fn from(inner: hiq_common::StockMargin) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockMargin {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("name", self.inner.name).unwrap();
        dict.set_item("trade_date", self.inner.trade_date).unwrap();
        dict.set_item("close", self.inner.close).unwrap();
        dict.set_item("chg_pct", self.inner.chg_pct).unwrap();
        dict.set_item("rz_ye", self.inner.rz_ye).unwrap();
        dict.set_item("rz_ye_zb", self.inner.rz_ye_zb).unwrap();
        dict.set_item("rz_che", self.inner.rz_che).unwrap();
        dict.set_item("rz_jme", self.inner.rz_jme).unwrap();
        dict.set_item("rq_ye", self.inner.rq_ye).unwrap();
        dict.set_item("rq_yl", self.inner.rq_yl).unwrap();
        dict.set_item("rq_mcl", self.inner.rq_mcl).unwrap();
        dict.set_item("rq_chl", self.inner.rq_chl).unwrap();
        dict.set_item("rq_jmg", self.inner.rq_jmg).unwrap();
        dict.set_item("rz_rq_ye", self.inner.rz_rq_ye).unwrap();
        dict.set_item("rz_rq_ye_cz", self.inner.rz_rq_ye_cz)
            .unwrap();

        dict.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockRtQuot {
    pub(crate) inner: hiq_common::StockRtQuot,
}

impl From<hiq_common::StockRtQuot> for StockRtQuot {
    fn from(inner: hiq_common::StockRtQuot) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockRtQuot {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("time", self.inner.time).unwrap();
        dict.set_item("last_close", self.inner.last_close).unwrap();
        dict.set_item("open", self.inner.open).unwrap();
        dict.set_item("high", self.inner.high).unwrap();
        dict.set_item("low", self.inner.low).unwrap();
        dict.set_item("last", self.inner.last).unwrap();
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
        dict.into()
    }
}
