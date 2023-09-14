mod bond;
mod fund;
mod stock;

mod ta;
mod types;

use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::collections::{BTreeSet, HashMap};

use crate::bond::{BlockBondFetch, BondFetch};
use crate::fund::{BlockFundFetch, FundFetch};
use crate::stock::{BlockStockFetch, StockFetch};
pub(crate) use pywqcmm::*;

use crate::ta::{calc_chip_dist, calc_cost, calc_winner};

/// Fetch trade_date.
#[pyfunction]
fn fetch_trade_date(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(rwqfetch::fetch_trade_date()
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_trade_date() -> PyResult<BTreeSet<i32>> {
    Ok(runtime()?
        .block_on(rwqfetch::fetch_trade_date())
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn fetch_next_trade_date(py: Python, date: NaiveDate) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(rwqfetch::fetch_next_trade_date(&date)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_next_trade_date(date: NaiveDate) -> PyResult<i32> {
    Ok(runtime()?
        .block_on(rwqfetch::fetch_next_trade_date(&date))
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn fetch_prev_trade_date(py: Python, date: NaiveDate) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(rwqfetch::fetch_prev_trade_date(&date)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_prev_trade_date(date: NaiveDate) -> PyResult<i32> {
    Ok(runtime()?
        .block_on(rwqfetch::fetch_prev_trade_date(&date))
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn fetch_is_trade_date(py: Python, date: NaiveDate) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(rwqfetch::fetch_is_trade_date(&date)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_is_trade_date(date: NaiveDate) -> PyResult<bool> {
    Ok(runtime()?
        .block_on(rwqfetch::fetch_is_trade_date(&date))
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn to_std_code(typ: i32, code: &str) -> PyResult<String> {
    let typ: rwqfetch::MarketType = typ.into();
    Ok(rwqfetch::to_std_code(typ, code))
}

#[pyfunction]
fn fetch_rt_quot<'a>(py: Python<'a>, code: Vec<&str>) -> PyResult<&'a PyAny> {
    let code: Vec<_> = code.into_iter().map(String::from).collect();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let code: Vec<_> = code.iter().map(|e| &e[..]).collect();
        Ok(rwqfetch::fetch_rt_quot(code)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?
            .into_iter()
            .map(|(key, value)| (key, RtQuot::from(value)))
            .collect::<HashMap<String, RtQuot>>())
    })
}
#[pyfunction]
fn block_fetch_rt_quot(code: Vec<&str>) -> PyResult<HashMap<String, RtQuot>> {
    Ok(runtime()?
        .block_on(async move {
            let code: Vec<_> = code.iter().map(|e| &e[..]).collect();
            rwqfetch::fetch_rt_quot(code).await
        })
        .map_err(|e| PyException::new_err(e.to_string()))?
        .into_iter()
        .map(|(key, value)| (key, RtQuot::from(value)))
        .collect::<HashMap<String, RtQuot>>())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pywqfetch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_next_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_next_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_prev_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_prev_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_is_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_is_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(to_std_code, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_rt_quot, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_rt_quot, m)?)?;
    m.add_function(wrap_pyfunction!(calc_chip_dist, m)?)?;
    m.add_function(wrap_pyfunction!(calc_winner, m)?)?;
    m.add_function(wrap_pyfunction!(calc_cost, m)?)?;
    m.add_class::<BondFetch>()?;
    m.add_class::<BlockBondFetch>()?;
    m.add_class::<FundFetch>()?;
    m.add_class::<BlockFundFetch>()?;
    m.add_class::<StockFetch>()?;
    m.add_class::<BlockStockFetch>()?;
    Ok(())
}
