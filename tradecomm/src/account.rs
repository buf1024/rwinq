use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::{Deal, Entrust, Position, Signal};

pub struct Account {
    pub id: String,

    pub status: i32,
    pub category: String,

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

    pub broker_fee: f32,   // 0025
    pub transfer_fee: f32, // 0002
    pub tax_fee: f32,      // 01

    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,

    pub position: HashMap<String, Position>,
    pub entrust: HashMap<String, Entrust>,

    // 成交 backtest
    pub deal: Vec<Deal>,
    pub signal: Vec<Signal>,
}
