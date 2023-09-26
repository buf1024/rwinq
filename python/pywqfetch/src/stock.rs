use crate::runtime;
use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::to_python;
use rwqfetch::Market;

/// 股票大盘指数（列举几个而已）
#[pyfunction]
pub(crate) fn fetch_index_info<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_index_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 指数k线数据
#[pyfunction]
pub(crate) fn fetch_index_bar<'a>(
    py: Python<'a>,
    code: &str,
    name: Option<&str>,
    freq: Option<i32>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<&'a PyAny> {
    let is_skip_rt = skip_rt.unwrap_or(true);

    let code = code.to_owned();
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let name = name.as_deref();
        let fr = freq.map(rwqfetch::BarFreq::from);

        to_python(
            &rwqfetch::fetch_stock_bar(&code[..], name, fr, start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 获取股票基本信息
#[pyfunction]
pub(crate) fn fetch_stock_info<'a>(py: Python<'a>, market: Option<i32>) -> PyResult<&'a PyAny> {
    let m = market.map(|v| Market::from(v));
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_info(m)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 获取融资融券股票代码
#[pyfunction]
pub(crate) fn fetch_stock_is_margin<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_is_margin()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票k线数据
#[pyfunction]
pub(crate) fn fetch_stock_bar<'a>(
    py: Python<'a>,
    code: &str,
    name: Option<&str>,
    freq: Option<i32>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<&'a PyAny> {
    let is_skip_rt = skip_rt.unwrap_or(true);

    let code = code.to_owned();
    let name = name.map(String::from);

    pyo3_asyncio::tokio::future_into_py(py, async move {
        let name = name.as_deref();
        let fr = freq.map(rwqfetch::BarFreq::from);

        to_python(
            &rwqfetch::fetch_stock_bar(&code[..], name, fr, start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票最新指标
#[pyfunction]
pub(crate) fn fetch_stock_index<'a>(py: Python<'a>, date: Option<NaiveDate>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_index(date)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}

/// 股票行业
#[pyfunction]
pub(crate) fn fetch_stock_industry<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_industry()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票行业详情
#[pyfunction]
pub(crate) fn fetch_stock_industry_detail<'a>(
    py: Python<'a>,
    code: Option<&str>,
    name: Option<&str>,
) -> PyResult<&'a PyAny> {
    let code = code.map(String::from);
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_industry_detail(code.as_deref(), name.as_deref())
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票行业k线数据，只有日频率
#[pyfunction]
pub(crate) fn fetch_stock_industry_daily<'a>(
    py: Python<'a>,
    code: &str,
    name: Option<&str>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<&'a PyAny> {
    let is_skip_rt = skip_rt.unwrap_or(true);

    let code = code.to_owned();
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_industry_daily(
                &code[..],
                name.as_deref(),
                start,
                end,
                is_skip_rt,
            )
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票概念
#[pyfunction]
pub(crate) fn fetch_stock_concept<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_concept()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票概念详情
#[pyfunction]
pub(crate) fn fetch_stock_concept_detail<'a>(
    py: Python<'a>,
    code: Option<&str>,
    name: Option<&str>,
) -> PyResult<&'a PyAny> {
    let code = code.map(String::from);
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_concept_detail(code.as_deref(), name.as_deref())
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票概念k线数据
#[pyfunction]
pub(crate) fn fetch_stock_concept_daily<'a>(
    py: Python<'a>,
    code: &str,
    name: Option<&str>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<&'a PyAny> {
    let is_skip_rt = skip_rt.unwrap_or(true);

    let code = code.to_owned();
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_concept_daily(
                &code[..],
                name.as_deref(),
                start,
                end,
                is_skip_rt,
            )
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 股票业绩报表
#[pyfunction]
pub(crate) fn fetch_stock_yjbb<'a>(py: Python<'a>, year: u16, season: u16) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_yjbb(year, season)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 融资融券
#[pyfunction]
pub(crate) fn fetch_stock_margin<'a>(
    py: Python<'a>,
    code: &str,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> PyResult<&'a PyAny> {
    let code = code.to_owned();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_margin(&code[..], start, end)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 热股排名
#[pyfunction]
pub(crate) fn fetch_stock_hot_rank<'a>(py: Python<'a>, code: &str) -> PyResult<&'a PyAny> {
    let code = code.to_owned();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_hot_rank(&code[..])
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 千股千评
#[pyfunction]
pub(crate) fn fetch_stock_comment<'a>(py: Python<'a>, codes: Option<Vec<String>>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_comment(codes)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 千股千评历史
#[pyfunction]
pub(crate) fn fetch_stock_comment_his<'a>(py: Python<'a>, code: String) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_stock_comment_his(code)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}

/// 股票大盘指数（列举几个而已）
#[pyfunction]
pub(crate) fn block_fetch_index_info() -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_index_info())
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 指数k线数据
#[pyfunction]
pub(crate) fn block_fetch_index_bar(
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
            .block_on(rwqfetch::fetch_stock_bar(
                code, name, fr, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 获取股票基本信息
#[pyfunction]
pub(crate) fn block_fetch_stock_info(market: Option<i32>) -> PyResult<PyObject> {
    let m = market.map(|v| Market::from(v));

    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_info(m))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 获取融资融券股票代码
#[pyfunction]
pub(crate) fn block_fetch_stock_is_margin() -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_is_margin())
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票k线数据
#[pyfunction]
pub(crate) fn block_fetch_stock_bar(
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
            .block_on(rwqfetch::fetch_stock_bar(
                code, name, fr, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票最新指标
#[pyfunction]
pub(crate) fn block_fetch_stock_index(date: Option<NaiveDate>) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_index(date))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}

/// 股票行业
#[pyfunction]
pub(crate) fn block_fetch_stock_industry() -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_industry())
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票行业详情
#[pyfunction]
pub(crate) fn block_fetch_stock_industry_detail(code: Option<&str>, name: Option<&str>) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_industry_detail(code, name))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票行业k线数据，只有日频率
#[pyfunction]
pub(crate) fn block_fetch_stock_industry_daily(
    code: &str,
    name: Option<&str>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<PyObject> {
    let is_skip_rt = skip_rt.unwrap_or(true);

    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_industry_daily(
                code, name, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票概念
#[pyfunction]
pub(crate) fn block_fetch_stock_concept() -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_concept())
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票概念详情
#[pyfunction]
pub(crate) fn block_fetch_stock_concept_detail(code: Option<&str>, name: Option<&str>) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_concept_detail(code, name))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票概念k线数据
#[pyfunction]
pub(crate) fn block_fetch_stock_concept_daily(
    code: &str,
    name: Option<&str>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<PyObject> {
    let is_skip_rt = skip_rt.unwrap_or(true);

    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_concept_daily(
                code, name, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 股票业绩报表
#[pyfunction]
pub(crate) fn block_fetch_stock_yjbb(year: u16, season: u16) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_yjbb(year, season))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 融资融券
#[pyfunction]
pub(crate) fn block_fetch_stock_margin(
    code: &str,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_margin(code, start, end))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 热股排名
#[pyfunction]
pub(crate) fn block_fetch_stock_hot_rank(code: &str) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_hot_rank(code))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 千股千评
#[pyfunction]
pub(crate) fn block_fetch_stock_comment(codes: Option<Vec<String>>) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_comment(codes))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 千股千评历史
#[pyfunction]
pub(crate) fn block_fetch_stock_comment_his(code: String) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_stock_comment_his(code))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
