pub mod entrust;
pub use entrust::*;

pub mod signal;
use serde::{Deserialize, Serialize};
pub use signal::*;

pub mod deal;
pub use deal::*;

pub mod position;
pub use position::*;

pub mod account;
pub use account::*;

pub mod event;
pub use event::*;

/// 交易类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeType {
    Buy = 0,
    Sell = 1,
    Cancel = 2,
}

impl Default for TradeType {
    fn default() -> Self {
        TradeType::Cancel
    }
}
