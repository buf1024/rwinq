mod strategy;
use serde::{Deserialize, Serialize};
pub use strategy::*;
use thiserror::Error;

mod util;

mod runner;
pub use runner::*;

mod mystrategy;
pub use mystrategy::{get_strategy, strategies};

pub mod ta;

pub use hiq_data::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Function \"{0}\" not implement")]
    NotImpl(&'static str),
    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyType {
    Bond = 1,
    Fund,
    Stock,
    Index,
    Concept,
    Industry,
}

impl From<i32> for StrategyType {
    fn from(v: i32) -> Self {
        match v {
            1 => StrategyType::Bond,
            2 => StrategyType::Fund,
            3 => StrategyType::Stock,
            4 => StrategyType::Index,
            5 => StrategyType::Concept,
            6 => StrategyType::Industry,
            _ => StrategyType::Stock,
        }
    }
}
impl From<&str> for StrategyType {
    fn from(v: &str) -> Self {
        match v {
            "bond" => StrategyType::Bond,
            "fund" => StrategyType::Fund,
            "stock" => StrategyType::Stock,
            "index" => StrategyType::Index,
            "concept" => StrategyType::Concept,
            "industry" => StrategyType::Industry,
            _ => StrategyType::Stock,
        }
    }
}

pub type Symbol = unsafe extern "C" fn() -> *mut libc::c_void;
pub const SYMBOL_NAME: &'static str = "new_strategy";
