use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::{runtime, to_python};

/// etf基金基本信息
#[pyfunction]
pub(crate) fn fetch_fund_info<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_fund_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// etf基金净值
#[pyfunction]
pub(crate) fn fetch_fund_net<'a>(
    py: Python<'a>,
    code: &str,
    name: Option<&str>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> PyResult<&'a PyAny> {
    let code = code.to_owned();
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let name = name.as_deref();

        to_python(
            &rwqfetch::fetch_fund_net(&code[..], name, start, end)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}

/// etf基金k线数据
#[pyfunction]
pub(crate) fn fetch_fund_bar<'a>(
    py: Python<'a>,
    code: &str,
    name: Option<&str>,
    freq: Option<i32>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<&'a PyAny> {
    let is_skip_rt = skip_rt.unwrap_or(true);
    let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));

    let code = code.to_owned();
    let name = name.map(String::from);
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let name = name.as_deref();

        to_python(
            &rwqfetch::fetch_fund_bar(&code[..], name, fr, start, end, is_skip_rt)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}

/// etf基金基本信息
#[pyfunction]
pub(crate) fn block_fetch_fund_info() -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_fund_info())
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// etf基金净值
#[pyfunction]
pub(crate) fn block_fetch_fund_net(
    code: &str,
    name: Option<&str>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_fund_net(code, name, start, end))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}

/// etf基金k线数据
#[pyfunction]
pub(crate) fn block_fetch_fund_bar(
    code: &str,
    name: Option<&str>,
    freq: Option<i32>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<PyObject> {
    let is_skip_rt = skip_rt.unwrap_or(true);
    let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));

    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_fund_bar(
                code, name, fr, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
