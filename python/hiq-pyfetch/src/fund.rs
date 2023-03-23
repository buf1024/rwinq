use std::sync::Arc;

use crate::{FundBar, FundInfo, FundNet};
use chrono::NaiveDate;
use hiq_pycommon::runtime;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass]
pub(crate) struct FundFetch {
    fetch: Arc<dyn hiq_fetch::FundFetch + Send + 'static>,
}

#[pymethods]
impl FundFetch {
    #[new]
    fn new() -> Self {
        FundFetch {
            fetch: Arc::new(hiq_fetch::fund_fetch()),
        }
    }
    /// etf基金基本信息
    fn fetch_fund_info<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_fund_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(FundInfo::from)
                .collect::<Vec<_>>())
        })
    }
    /// etf基金净值
    fn fetch_fund_net<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let name = name.as_deref();
            Ok(fetch
                .fetch_fund_net(&code[..], name, start, end)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(FundNet::from)
                .collect::<Vec<_>>())
        })
    }

    /// etf基金k线数据
    fn fetch_fund_bar<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: Option<&str>,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let fr: Option<hiq_fetch::BarFreq> = if let Some(v) = freq {
                Some(v.into())
            } else {
                None
            };
            let name = name.as_deref();
            let bar: FundBar = fetch
                .fetch_fund_bar(&code[..], name, fr, start, end, skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(bar)
        })
    }
}

#[pyclass]
pub(crate) struct BlockFundFetch {
    fetch: Box<dyn hiq_fetch::FundFetch + Send + 'static>,
}

#[pymethods]
impl BlockFundFetch {
    #[new]
    fn new() -> Self {
        BlockFundFetch {
            fetch: Box::new(hiq_fetch::fund_fetch()),
        }
    }
    /// etf基金基本信息
    fn fetch_fund_info(&self) -> PyResult<Vec<FundInfo>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_fund_info())
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(FundInfo::from)
            .collect::<Vec<_>>())
    }
    /// etf基金净值
    fn fetch_fund_net(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> PyResult<Vec<FundNet>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_fund_net(code, name, start, end))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(FundNet::from)
            .collect::<Vec<_>>())
    }

    /// etf基金k线数据
    fn fetch_fund_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> PyResult<FundBar> {
        let fr: Option<hiq_fetch::BarFreq> = if let Some(v) = freq {
            Some(v.into())
        } else {
            None
        };
        Ok(runtime()?
            .block_on(
                self.fetch
                    .fetch_fund_bar(code, name, fr, start, end, skip_rt),
            )
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
}
