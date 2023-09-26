use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::{runtime, to_python};

/// 获取可转债基本信息
#[pyfunction]
pub(crate) fn fetch_bond_info<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_bond_info()
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
/// 获取可转债日线
#[pyfunction]
pub(crate) fn fetch_bond_bar<'a>(
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

    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_bond_bar(
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
            .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}

/// 获取可转债基本信息
#[pyfunction]
pub(crate) fn block_fetch_bond_info() -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_bond_info())
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
/// 获取可转债日线
#[pyfunction]
pub(crate) fn block_fetch_bond_bar(
    code: &str,
    name: &str,
    stock_code: &str,
    stock_name: &str,
    freq: Option<i32>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: Option<bool>,
) -> PyResult<PyObject> {
    let is_skip_rt = skip_rt.unwrap_or(true);
    let fr: Option<rwqfetch::BarFreq> = freq.map_or(None, |v| Some(v.into()));

    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_bond_bar(
                code, name, stock_code, stock_name, fr, start, end, is_skip_rt,
            ))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
