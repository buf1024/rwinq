use std::collections::HashMap;

use rwqcmm::MarketType;
use serde::{Deserialize, Serialize};

use crate::{Deal, Entrust, Position, Signal, TradeTime, TradeType, Uuid};

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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Account {
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub id: Uuid,
    pub status: AccountStatus,
    pub typ: MarketType,
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

impl Account {
    pub fn get_position_volume(&self, code: &str) -> (u32, u32) {
        if !self.position.contains_key(code) {
            return (0, 0);
        }

        let position = self.position.get(code).unwrap();
        return (position.volume, position.volume_available);
    }

    pub fn get_active_entrust(&self, code: &str) -> Vec<Entrust> {
        self.entrust
            .values()
            .filter(|entrust| entrust.code == String::from(code))
            .map(|entrust| entrust.clone())
            .collect()
    }

    pub fn get_est_fee(&self, typ: TradeType, code: &str, price: f32, volume: u32) -> f32 {
        let total = price * volume as f32;
        let mut broker_fee = total * self.broker_fee;

        if broker_fee < 5.0 {
            broker_fee = 5.0;
        }
        let mut tax_fee = 0.0;
        if matches!(self.typ, MarketType::Stock) {
            match typ {
                TradeType::Buy => {
                    if code.contains("sh") {
                        tax_fee = total * self.transfer_fee;
                    }
                }
                TradeType::Sell => {
                    if code.contains("sh") {
                        tax_fee = total * self.tax_fee;
                    }
                }
                TradeType::Cancel => {}
            }
        }
        let fee = broker_fee + tax_fee;
        (fee * 100.0).round() / 100.0
    }

    pub fn get_est_cost(&self, typ: TradeType, code: &str, price: f32, volume: u32) -> f32 {
        let fee = self.get_est_fee(typ, code, price, volume) + price * volume as f32;
        (fee * 100.0).round() / 100.0
    }
}
