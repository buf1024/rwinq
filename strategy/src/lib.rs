use std::collections::HashMap;

use thiserror::Error;

pub mod util;

pub mod runner;
pub use runner::*;

mod mystrategy;
// pub use mystrategy::{get_strategy, strategies};

pub mod ta;

pub use rwqdata::*;

pub mod broker;
pub mod risk;
pub mod select;
pub mod trade;

pub mod context;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Function \"{0}\" not implement")]
    NotImpl(&'static str),
    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub type  Params = HashMap<String, String>;

/// 策略导出的函数
pub type Symbol = unsafe extern "C" fn() -> *mut libc::c_void;
/// 选股策略导出的函数名称
pub const SYMBOL_STRATEGY_SELECT: &'static str = "new_strategy";
/// 交易策略导出的函数名称
pub const SYMBOL_STRATEGY_TRADE: &'static str = "new_strategy";
/// 风控策略导出的函数名称
pub const SYMBOL_RISK: &'static str = "new_risk";
/// 券商导出的函数名称
pub const SYMBOL_BROKER: &'static str = "new_broker";
