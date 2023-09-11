use crate::{
    StockBar, StockConcept, StockConceptBar, StockConceptDetail, StockIndex, StockIndustry,
    StockIndustryBar, StockIndustryDetail, StockInfo, StockMargin, StockRtQuot, StockYJBB,
};
use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::{runtime, StockHotRank};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[pyclass]
pub(crate) struct StockFetch {
    fetch: Arc<rwqfetch::StockFetch>,
}

#[pymethods]
impl StockFetch {
    #[new]
    fn new() -> Self {
        StockFetch {
            fetch: Arc::new(rwqfetch::stock_fetch()),
        }
    }
    /// 股票大盘指数（列举几个而已）
    fn fetch_index_info<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_index_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockInfo::from)
                .collect::<Vec<_>>())
        })
    }
    /// 指数k线数据
    fn fetch_index_bar<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: Option<&str>,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<&'a PyAny> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let name = name.as_deref();
            let fr: Option<rwqfetch::BarFreq> = if let Some(v) = freq {
                Some(v.into())
            } else {
                None
            };
            let bar: StockBar = fetch
                .fetch_stock_bar(&code[..], name, fr, start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(bar)
        })
    }
    /// 获取股票基本信息
    fn fetch_stock_info<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockInfo::from)
                .collect::<Vec<_>>())
        })
    }
    /// 获取融资融券股票代码
    fn fetch_stock_is_margin<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_is_margin()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?)
        })
    }
    /// 股票k线数据
    fn fetch_stock_bar<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: Option<&str>,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<&'a PyAny> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        let name = name.map(String::from);

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let name = name.as_deref();
            let fr: Option<rwqfetch::BarFreq> = if let Some(v) = freq {
                Some(v.into())
            } else {
                None
            };
            let bar: StockBar = fetch
                .fetch_stock_bar(&code[..], name, fr, start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(bar)
        })
    }
    /// 股票最新指标
    fn fetch_stock_index<'a>(
        &self,
        py: Python<'a>,
        date: Option<NaiveDate>,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_index(date)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(|(key, value)| (key, StockIndex::from(value)))
                .collect::<HashMap<String, StockIndex>>())
        })
    }

    /// 股票行业
    fn fetch_stock_industry<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_industry()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockIndustry::from)
                .collect::<Vec<_>>())
        })
    }
    /// 股票行业详情
    fn fetch_stock_industry_detail<'a>(
        &self,
        py: Python<'a>,
        code: Option<&str>,
        name: Option<&str>,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.map(String::from);
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_industry_detail(code.as_deref(), name.as_deref())
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockIndustryDetail::from)
                .collect::<Vec<_>>())
        })
    }
    /// 股票行业k线数据，只有日频率
    fn fetch_stock_industry_daily<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<&'a PyAny> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let bar: StockIndustryBar = fetch
                .fetch_stock_industry_daily(&code[..], name.as_deref(), start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(bar)
        })
    }
    /// 股票概念
    fn fetch_stock_concept<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_concept()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockConcept::from)
                .collect::<Vec<_>>())
        })
    }
    /// 股票概念详情
    fn fetch_stock_concept_detail<'a>(
        &self,
        py: Python<'a>,
        code: Option<&str>,
        name: Option<&str>,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.map(String::from);
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_concept_detail(code.as_deref(), name.as_deref())
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockConceptDetail::from)
                .collect::<Vec<_>>())
        })
    }
    /// 股票概念k线数据
    fn fetch_stock_concept_daily<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<&'a PyAny> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        let name = name.map(String::from);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let bar: StockConceptBar = fetch
                .fetch_stock_concept_daily(&code[..], name.as_deref(), start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(bar)
        })
    }
    /// 股票业绩报表
    fn fetch_stock_yjbb<'a>(&self, py: Python<'a>, year: u16, season: u16) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_yjbb(year, season)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockYJBB::from)
                .collect::<Vec<_>>())
        })
    }
    /// 融资融券
    fn fetch_stock_margin<'a>(
        &self,
        py: Python<'a>,
        code: &str,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(fetch
                .fetch_stock_margin(&code[..], start, end)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(StockMargin::from)
                .collect::<Vec<_>>())
        })
    }
    /// 热股排名
    fn fetch_stock_hot_rank<'a>(&self, py: Python<'a>, code: &str) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let hot_rank: StockHotRank = fetch
                .fetch_stock_hot_rank(&code[..])
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into();
            Ok(hot_rank)
        })
    }
    /// 实时行情
    fn fetch_stock_rt_quot<'a>(&self, py: Python<'a>, code: Vec<&str>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code: Vec<_> = code.into_iter().map(String::from).collect();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let code: Vec<_> = code.iter().map(|e| &e[..]).collect();
            Ok(fetch
                .fetch_stock_rt_quot(code)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?
                .into_iter()
                .map(|(key, value)| (key, StockRtQuot::from(value)))
                .collect::<HashMap<String, StockRtQuot>>())
        })
    }
}

#[pyclass]
pub(crate) struct BlockStockFetch {
    fetch: Box<rwqfetch::StockFetch>,
}

#[pymethods]
impl BlockStockFetch {
    #[new]
    fn new() -> Self {
        BlockStockFetch {
            fetch: Box::new(rwqfetch::stock_fetch()),
        }
    }
    /// 股票大盘指数（列举几个而已）
    fn fetch_index_info(&self) -> PyResult<Vec<StockInfo>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_index_info())
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockInfo::from)
            .collect())
    }
    /// 指数k线数据
    fn fetch_index_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<StockBar> {
        let is_skip_rt = skip_rt.unwrap_or(true);

        let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));
        Ok(runtime()?
            .block_on(
                self.fetch
                    .fetch_stock_bar(code, name, fr, start, end, is_skip_rt),
            )
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
    /// 获取股票基本信息
    fn fetch_stock_info(&self) -> PyResult<Vec<StockInfo>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_info())
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockInfo::from)
            .collect())
    }
    /// 获取融资融券股票代码
    fn fetch_stock_is_margin(&self) -> PyResult<HashSet<String>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_is_margin())
            .map_err(|e| PyException::new_err(e.to_string()))?)
    }
    /// 股票k线数据
    fn fetch_stock_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<i32>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<StockBar> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));

        Ok(runtime()?
            .block_on(
                self.fetch
                    .fetch_stock_bar(code, name, fr, start, end, is_skip_rt),
            )
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
    /// 股票最新指标
    fn fetch_stock_index(&self, date: Option<NaiveDate>) -> PyResult<HashMap<String, StockIndex>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_index(date))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(|(key, value)| (key, StockIndex::from(value)))
            .collect())
    }

    /// 股票行业
    fn fetch_stock_industry(&self) -> PyResult<Vec<StockIndustry>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_industry())
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockIndustry::from)
            .collect())
    }
    /// 股票行业详情
    fn fetch_stock_industry_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> PyResult<Vec<StockIndustryDetail>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_industry_detail(code, name))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockIndustryDetail::from)
            .collect())
    }
    /// 股票行业k线数据，只有日频率
    fn fetch_stock_industry_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<StockIndustryBar> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        Ok(runtime()?
            .block_on(
                self.fetch
                    .fetch_stock_industry_daily(code, name, start, end, is_skip_rt),
            )
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
    /// 股票概念
    fn fetch_stock_concept(&self) -> PyResult<Vec<StockConcept>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_concept())
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockConcept::from)
            .collect())
    }
    /// 股票概念详情
    fn fetch_stock_concept_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> PyResult<Vec<StockConceptDetail>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_concept_detail(code, name))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockConceptDetail::from)
            .collect())
    }
    /// 股票概念k线数据
    fn fetch_stock_concept_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<StockConceptBar> {
        let is_skip_rt = skip_rt.unwrap_or(true);
        Ok(runtime()?
            .block_on(
                self.fetch
                    .fetch_stock_concept_daily(code, name, start, end, is_skip_rt),
            )
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
    /// 股票业绩报表
    fn fetch_stock_yjbb(&self, year: u16, season: u16) -> PyResult<Vec<StockYJBB>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_yjbb(year, season))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockYJBB::from)
            .collect())
    }
    /// 融资融券
    fn fetch_stock_margin(
        &self,
        code: &str,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> PyResult<Vec<StockMargin>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_margin(code, start, end))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(StockMargin::from)
            .collect())
    }
    /// 热股排名
    fn fetch_stock_hot_rank(&self, code: &str) -> PyResult<StockHotRank> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_hot_rank(code))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into())
    }
    /// 实时行情
    fn fetch_stock_rt_quot(&self, code: Vec<&str>) -> PyResult<HashMap<String, StockRtQuot>> {
        Ok(runtime()?
            .block_on(self.fetch.fetch_stock_rt_quot(code))
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(|(key, value)| (key, StockRtQuot::from(value)))
            .collect())
    }
}
