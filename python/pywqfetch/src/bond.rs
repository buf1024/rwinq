use std::sync::Arc;

use crate::{BondBar, BondInfo};
use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::runtime;

#[pyclass]
pub(crate) struct BondFetch {
    fetch: Arc<rwqfetch::BondFetch>,
}

#[pymethods]
impl BondFetch {
    #[new]
    fn new() -> Self {
        BondFetch {
            fetch: Arc::new(rwqfetch::bond_fetch()),
        }
    }
    /// 获取可转债基本信息
    fn fetch_bond_info<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_bond_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(BondInfo::from)
                .collect::<Vec<_>>())
        })
    }
    /// 获取可转债日线
    fn fetch_bond_bar<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: &str,
        stock_code: &str,
        stock_name: &str,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<&'a PyAny> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));
        let code = code.to_owned();
        let name = name.to_owned();
        let stock_code = stock_code.to_owned();
        let stock_name = stock_name.to_owned();

        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let bar: BondBar = fetch
                .fetch_bond_bar(
                    &code[..],
                    &name[..],
                    &stock_code[..],
                    &stock_name[..],
                    fr,
                    start,
                    end,
                    is_skip_rt,
                )
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(bar)
        })
    }
}

#[pyclass]
pub(crate) struct BlockBondFetch {
    fetch: Box<rwqfetch::BondFetch>,
}

#[pymethods]
impl BlockBondFetch {
    #[new]
    fn new() -> Self {
        BlockBondFetch {
            fetch: Box::new(rwqfetch::bond_fetch()),
        }
    }
    /// 获取可转债基本信息
    fn fetch_bond_info(&self) -> PyResult<Vec<BondInfo>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_bond_info())
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(BondInfo::from)
            .collect())
    }
    /// 获取可转债日线
    fn fetch_bond_bar(
        &self,
        code: &str,
        name: &str,
        stock_code: &str,
        stock_name: &str,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<BondBar> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));
        Ok(runtime()?
            .block_on(self.fetch.fetch_bond_bar(
                code, name, stock_code, stock_name, fr, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
}
