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
pub(crate) use hiq_pycommon::*;

/// Fetch trade_date.
#[pyfunction]
fn fetch_trade_date(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(hiq_fetch::fetch_trade_date()
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_trade_date() -> PyResult<BTreeSet<i32>> {
    Ok(runtime()?
        .block_on(hiq_fetch::fetch_trade_date())
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn fetch_next_trade_date(py: Python, date: NaiveDate) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(hiq_fetch::fetch_next_trade_date(&date)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_next_trade_date(date: NaiveDate) -> PyResult<i32> {
    Ok(runtime()?
        .block_on(hiq_fetch::fetch_next_trade_date(&date))
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn fetch_prev_trade_date(py: Python, date: NaiveDate) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(hiq_fetch::fetch_prev_trade_date(&date)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?)
    })
}

#[pyfunction]
fn block_fetch_prev_trade_date(date: NaiveDate) -> PyResult<i32> {
    Ok(runtime()?
        .block_on(hiq_fetch::fetch_prev_trade_date(&date))
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

#[pyfunction]
fn to_std_code(typ: i32, code: &str) -> PyResult<String> {
    let typ: hiq_fetch::MarketType = typ.into();
    Ok(hiq_fetch::to_std_code(typ, code))
}

/// A Python module implemented in Rust.
#[pymodule]
fn hiq_pyfetch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_next_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_next_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_prev_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_prev_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(to_std_code, m)?)?;
    m.add_class::<BondFetch>()?;
    m.add_class::<BlockBondFetch>()?;
    m.add_class::<FundFetch>()?;
    m.add_class::<BlockFundFetch>()?;
    m.add_class::<StockFetch>()?;
    m.add_class::<BlockStockFetch>()?;
    Ok(())
}
