mod bond;
mod fund;
mod stock;

use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::collections::BTreeSet;

use crate::bond::{BlockBondFetch, BondFetch};
use crate::fund::{BlockFundFetch, FundFetch};
use crate::stock::{BlockStockFetch, StockFetch};
pub(crate) use pywqcmm::*;

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
    m.add_class::<BondFetch>()?;
    m.add_class::<BlockBondFetch>()?;
    m.add_class::<FundFetch>()?;
    m.add_class::<BlockFundFetch>()?;
    m.add_class::<StockFetch>()?;
    m.add_class::<BlockStockFetch>()?;
    Ok(())
}
