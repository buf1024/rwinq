mod bond;
mod fund;
mod stock;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::collections::HashSet;

use crate::bond::{BondFetch, BlockBondFetch};
use crate::fund::{FundFetch, BlockFundFetch};
use crate::stock::{StockFetch, BlockStockFetch};
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
fn block_fetch_trade_date() -> PyResult<HashSet<i32>> {
    Ok(runtime()?
        .block_on(hiq_fetch::fetch_trade_date())
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
    m.add_function(wrap_pyfunction!(to_std_code, m)?)?;
    m.add_class::<BondFetch>()?;
    m.add_class::<BlockBondFetch>()?;
    m.add_class::<FundFetch>()?;
    m.add_class::<BlockFundFetch>()?;
    m.add_class::<StockFetch>()?;
    m.add_class::<BlockStockFetch>()?;
    Ok(())
}
