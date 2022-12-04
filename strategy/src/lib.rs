mod strategy;
use serde::{Serialize, Deserialize};
pub use strategy::*;
use thiserror::Error;

mod runner;
pub use runner::*;

mod mystrategy;
pub use mystrategy::{get_strategy, strategies};

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

pub type Symbol = unsafe extern "C" fn() -> *mut libc::c_void;
pub const SYMBOL_NAME: &'static str = "new_strategy";
