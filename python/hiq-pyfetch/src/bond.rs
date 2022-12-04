use std::sync::Arc;

use crate::{BondBar, BondInfo};
use chrono::NaiveDate;
use hiq_pycommon::runtime;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass]
pub(crate) struct BondFetch {
    fetch: Arc<dyn hiq_fetch::BondFetch + Sync + Send + 'static>,
}

#[pymethods]
impl BondFetch {
    #[new]
    fn new() -> Self {
        BondFetch {
            fetch: Arc::new(hiq_fetch::bond_fetch()),
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
    ) -> PyResult<&'a PyAny> {
        let code = code.to_owned();
        let name = name.to_owned();
        let stock_code = stock_code.to_owned();
        let stock_name = stock_name.to_owned();

        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let fr: Option<hiq_fetch::BarFreq> = if let Some(v) = freq {
                Some(v.into())
            } else {
                None
            };

            let bar: BondBar = fetch
                .fetch_bond_bar(
                    &code[..],
                    &name[..],
                    &stock_code[..],
                    &stock_name[..],
                    fr,
                    start,
                    end,
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
    fetch: Box<dyn hiq_fetch::BondFetch + Send + 'static>,
}

#[pymethods]
impl BlockBondFetch {
    #[new]
    fn new() -> Self {
        BlockBondFetch {
            fetch: Box::new(hiq_fetch::bond_fetch()),
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
    ) -> PyResult<BondBar> {
        let fr: Option<hiq_fetch::BarFreq> = if let Some(v) = freq {
            Some(v.into())
        } else {
            None
        };
        Ok(runtime()?
            .block_on(
                self.fetch
                    .fetch_bond_bar(code, name, stock_code, stock_name, fr, start, end),
            )
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
}
