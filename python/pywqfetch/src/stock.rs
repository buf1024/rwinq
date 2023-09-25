use crate::runtime;
use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::to_python;
use rwqfetch::Market;
use std::sync::Arc;

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
            to_python(
                &fetch
                    .fetch_index_info()
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            let fr = freq.map(rwqfetch::BarFreq::from);

            to_python(
                &fetch
                    .fetch_stock_bar(&code[..], name, fr, start, end, is_skip_rt)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 获取股票基本信息
    fn fetch_stock_info<'a>(&self, py: Python<'a>, market: Option<i32>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let m = market.map(|v| Market::from(v));
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_info(m)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 获取融资融券股票代码
    fn fetch_stock_is_margin<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_is_margin()
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            let fr = freq.map(rwqfetch::BarFreq::from);

            to_python(
                &fetch
                    .fetch_stock_bar(&code[..], name, fr, start, end, is_skip_rt)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            to_python(
                &fetch
                    .fetch_stock_index(date)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }

    /// 股票行业
    fn fetch_stock_industry<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_industry()
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            to_python(
                &fetch
                    .fetch_stock_industry_detail(code.as_deref(), name.as_deref())
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            to_python(
                &fetch
                    .fetch_stock_industry_daily(&code[..], name.as_deref(), start, end, is_skip_rt)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 股票概念
    fn fetch_stock_concept<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_concept()
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            to_python(
                &fetch
                    .fetch_stock_concept_detail(code.as_deref(), name.as_deref())
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            to_python(
                &fetch
                    .fetch_stock_concept_daily(&code[..], name.as_deref(), start, end, is_skip_rt)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 股票业绩报表
    fn fetch_stock_yjbb<'a>(&self, py: Python<'a>, year: u16, season: u16) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_yjbb(year, season)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
            to_python(
                &fetch
                    .fetch_stock_margin(&code[..], start, end)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 热股排名
    fn fetch_stock_hot_rank<'a>(&self, py: Python<'a>, code: &str) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        let code = code.to_owned();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_hot_rank(&code[..])
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 千股千评
    fn fetch_stock_comment<'a>(
        &self,
        py: Python<'a>,
        codes: Option<Vec<String>>,
    ) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_comment(codes)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
        })
    }
    /// 千股千评历史
    fn fetch_stock_comment_his<'a>(&self, py: Python<'a>, code: String) -> PyResult<&'a PyAny> {
        let fetch = self.fetch.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            to_python(
                &fetch
                    .fetch_stock_comment_his(code)
                    .await
                    .map_err(|e| PyException::new_err(e.to_string()))?,
            )
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
    fn fetch_index_info(&self) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_index_info())
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
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
    ) -> PyResult<PyObject> {
        let is_skip_rt = skip_rt.unwrap_or(true);

        let fr = freq.map(rwqfetch::BarFreq::from);

        to_python(
            &runtime()?
                .block_on(
                    self.fetch
                        .fetch_stock_bar(code, name, fr, start, end, is_skip_rt),
                )
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 获取股票基本信息
    fn fetch_stock_info(&self, market: Option<i32>) -> PyResult<PyObject> {
        let m = market.map(|v| Market::from(v));

        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_info(m))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 获取融资融券股票代码
    fn fetch_stock_is_margin(&self) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_is_margin())
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
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
    ) -> PyResult<PyObject> {
        let is_skip_rt = skip_rt.unwrap_or(true);

        let fr = freq.map(rwqfetch::BarFreq::from);

        to_python(
            &runtime()?
                .block_on(
                    self.fetch
                        .fetch_stock_bar(code, name, fr, start, end, is_skip_rt),
                )
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票最新指标
    fn fetch_stock_index(&self, date: Option<NaiveDate>) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_index(date))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }

    /// 股票行业
    fn fetch_stock_industry(&self) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_industry())
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票行业详情
    fn fetch_stock_industry_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_industry_detail(code, name))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票行业k线数据，只有日频率
    fn fetch_stock_industry_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<PyObject> {
        let is_skip_rt = skip_rt.unwrap_or(true);

        to_python(
            &runtime()?
                .block_on(
                    self.fetch
                        .fetch_stock_industry_daily(code, name, start, end, is_skip_rt),
                )
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票概念
    fn fetch_stock_concept(&self) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_concept())
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票概念详情
    fn fetch_stock_concept_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_concept_detail(code, name))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票概念k线数据
    fn fetch_stock_concept_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: Option<bool>,
    ) -> PyResult<PyObject> {
        let is_skip_rt = skip_rt.unwrap_or(true);

        to_python(
            &runtime()?
                .block_on(
                    self.fetch
                        .fetch_stock_concept_daily(code, name, start, end, is_skip_rt),
                )
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 股票业绩报表
    fn fetch_stock_yjbb(&self, year: u16, season: u16) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_yjbb(year, season))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 融资融券
    fn fetch_stock_margin(
        &self,
        code: &str,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_margin(code, start, end))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 热股排名
    fn fetch_stock_hot_rank(&self, code: &str) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_hot_rank(code))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 千股千评
    fn fetch_stock_comment(&self, codes: Option<Vec<String>>) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_comment(codes))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
    /// 千股千评历史
    fn fetch_stock_comment_his(&self, code: String) -> PyResult<PyObject> {
        to_python(
            &runtime()?
                .block_on(self.fetch.fetch_stock_comment_his(code))
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    }
}
