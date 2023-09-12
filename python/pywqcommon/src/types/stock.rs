use crate::Bar;
use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPy, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    pub(crate) inner: rwqcmm::StockInfo,
}

impl From<rwqcmm::StockInfo> for StockInfo {
    fn from(inner: rwqcmm::StockInfo) -> Self {
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
    pub(crate) inner: rwqcmm::StockBar,
}

impl From<rwqcmm::StockBar> for StockBar {
    fn from(inner: rwqcmm::StockBar) -> Self {
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
    pub(crate) inner: rwqcmm::StockIndex,
}

impl From<rwqcmm::StockIndex> for StockIndex {
    fn from(inner: rwqcmm::StockIndex) -> Self {
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
    pub(crate) inner: rwqcmm::StockIndustry,
}

impl From<rwqcmm::StockIndustry> for StockIndustry {
    fn from(inner: rwqcmm::StockIndustry) -> Self {
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
    pub(crate) inner: rwqcmm::StockIndustryDetail,
}

impl From<rwqcmm::StockIndustryDetail> for StockIndustryDetail {
    fn from(inner: rwqcmm::StockIndustryDetail) -> Self {
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
    pub(crate) inner: rwqcmm::StockIndustryBar,
}

impl From<rwqcmm::StockIndustryBar> for StockIndustryBar {
    fn from(inner: rwqcmm::StockIndustryBar) -> Self {
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
    pub(crate) inner: rwqcmm::StockConcept,
}

impl From<rwqcmm::StockConcept> for StockConcept {
    fn from(inner: rwqcmm::StockConcept) -> Self {
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
    pub(crate) inner: rwqcmm::StockConceptDetail,
}

impl From<rwqcmm::StockConceptDetail> for StockConceptDetail {
    fn from(inner: rwqcmm::StockConceptDetail) -> Self {
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
    pub(crate) inner: rwqcmm::StockConceptBar,
}

impl From<rwqcmm::StockConceptBar> for StockConceptBar {
    fn from(inner: rwqcmm::StockConceptBar) -> Self {
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
    pub(crate) inner: rwqcmm::StockYJBB,
}

impl From<rwqcmm::StockYJBB> for StockYJBB {
    fn from(inner: rwqcmm::StockYJBB) -> Self {
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
    pub(crate) inner: rwqcmm::StockMargin,
}

impl From<rwqcmm::StockMargin> for StockMargin {
    fn from(inner: rwqcmm::StockMargin) -> Self {
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
pub struct StockHotRank {
    pub(crate) inner: rwqcmm::StockHotRank,
}

impl From<rwqcmm::StockHotRank> for StockHotRank {
    fn from(inner: rwqcmm::StockHotRank) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for StockHotRank {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("code", self.inner.code).unwrap();
        dict.set_item("market_all_count", self.inner.market_all_count)
            .unwrap();
        dict.set_item("rank", self.inner.rank).unwrap();
        dict.set_item("rank_chg", self.inner.rank_chg).unwrap();
        dict.set_item("calc_time", self.inner.calc_time).unwrap();

        dict.into()
    }
}
