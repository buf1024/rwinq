use std::collections::HashMap;

use rwqcmm::MarketType;
use serde::{Deserialize, Serialize};

use crate::{Deal, Entrust, Position, Signal, TradeTime, Uuid};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    Backtest,
    Simulation,
    Real,
}

impl Default for AccountKind {
    fn default() -> Self {
        Self::Backtest
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccountStatus {
    Running,
    Stop,
}

impl Default for AccountStatus {
    fn default() -> Self {
        Self::Stop
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub id: Uuid,
    pub status: AccountStatus,
    pub typ: Option<MarketType>,
    pub kind: AccountKind,

    pub cash_init: f32,
    pub cash_available: f32,
    pub cash_frozen: f32,
    pub total_net_value: f32,

    pub total_hold_value: f32,
    pub cost: f32,        // 持仓陈本
    pub profit: f32,      // 持仓盈亏
    pub profit_rate: f32, // 持仓盈比例

    pub close_profit: f32, // 平仓盈亏

    pub total_profit: f32,      // 总盈亏
    pub total_profit_rate: f32, // 总盈亏比例

    pub broker_fee: f32,   // 0.00025
    pub transfer_fee: f32, // 0.00002
    pub tax_fee: f32,      // 0.001

    #[serde(
        serialize_with = "crate::opt_trade_time_serialize",
        deserialize_with = "crate::opt_trade_time_deserialize"
    )]
    pub start_time: Option<TradeTime>,
    #[serde(
        serialize_with = "crate::opt_trade_time_serialize",
        deserialize_with = "crate::opt_trade_time_deserialize"
    )]
    pub end_time: Option<TradeTime>,

    pub position: HashMap<String, Position>,
    pub entrust: HashMap<String, Entrust>,

    // 成交 backtest
    pub deal: Vec<Deal>,
    pub signal: Vec<Signal>,
}
