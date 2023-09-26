mod bond;
mod fund;
mod stock;

mod ta;

use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::collections::BTreeSet;
// use tracing::Event;
// use tracing_error::ErrorLayer;
// use tracing_subscriber::util::SubscriberInitExt;
// use tracing_subscriber::{layer::SubscriberExt, Layer};

use crate::bond::*;
use crate::fund::*;
use crate::stock::*;
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
fn fetch_rt_quot<'a>(py: Python<'a>, code: Vec<String>) -> PyResult<&'a PyAny> {
    let code: Vec<_> = code.into_iter().map(String::from).collect();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        to_python(
            &rwqfetch::fetch_rt_quot(&code)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?,
        )
    })
}
#[pyfunction]
fn block_fetch_rt_quot(code: Vec<String>) -> PyResult<PyObject> {
    to_python(
        &runtime()?
            .block_on(rwqfetch::fetch_rt_quot(&code))
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}

// struct LogTracingLayer {}

// impl LogTracingLayer {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// impl<S> Layer<S> for LogTracingLayer
// where
//     S: tracing::Subscriber,
// {
//     fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
//         println!("Got event!");
//         println!("  level={:?}", event.metadata().level());
//         println!("  target={:?}", event.metadata().target());
//         println!("  name={:?}", event.metadata().name());
//         for field in event.fields() {
//             println!("  field={}", field.name());
//         }
//     }
// }

/// A Python module implemented in Rust.
#[pymodule]
fn pywqfetch(_py: Python, m: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    // tracing_subscriber::registry()
    //     .with(LogTracingLayer::new())
    //     .with(ErrorLayer::default())
    //     .init();
    m.add_function(wrap_pyfunction!(fetch_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_trade_date, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_next_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_next_trade_date, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_prev_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_prev_trade_date, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_is_trade_date, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_is_trade_date, m)?)?;

    // quot
    m.add_function(wrap_pyfunction!(fetch_rt_quot, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_rt_quot, m)?)?;

    // misc
    m.add_function(wrap_pyfunction!(to_std_code, m)?)?;

    // chip dist
    m.add_function(wrap_pyfunction!(calc_chip_dist, m)?)?;
    m.add_function(wrap_pyfunction!(calc_winner, m)?)?;
    m.add_function(wrap_pyfunction!(calc_cost, m)?)?;

    // bond
    m.add_function(wrap_pyfunction!(fetch_bond_info, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_bond_info, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_bond_bar, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_bond_bar, m)?)?;

    // fund
    m.add_function(wrap_pyfunction!(fetch_fund_info, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_fund_info, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_fund_net, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_fund_net, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_fund_bar, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_fund_bar, m)?)?;

    // stock
    m.add_function(wrap_pyfunction!(fetch_index_info, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_index_info, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_index_bar, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_index_bar, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_info, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_info, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_is_margin, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_is_margin, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_bar, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_bar, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_index, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_index, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_stock_industry, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_industry, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_industry_detail, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_industry_detail, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_industry_daily, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_industry_daily, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_concept, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_concept, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_concept_detail, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_concept_detail, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_concept_daily, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_concept_daily, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_yjbb, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_yjbb, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_margin, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_margin, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_hot_rank, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_hot_rank, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_comment, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_comment, m)?)?;

    m.add_function(wrap_pyfunction!(fetch_stock_comment_his, m)?)?;
    m.add_function(wrap_pyfunction!(block_fetch_stock_comment_his, m)?)?;

    Ok(())
}
