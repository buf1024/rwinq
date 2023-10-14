use rwqcmm::RtQuot;
use serde::{Deserialize, Serialize};

use crate::{Entrust, Position, Signal, TradeTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotEvent {
    Start,
    MorningOpen,
    MorningClose,
    NoonOpen,
    NoonClose,
    End,
    Quot(RtQuot),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuotOpts {
    // pub typ: MarketType,
    /// seconds
    pub freq: i64,
    pub start_date: Option<TradeTime>,
    pub end_date: Option<TradeTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Event {
    /// 交易信号, 发出: strategy/risk
    Signal(Signal),
    /// 行情订阅, 发出: strategy
    Subscribe(Vec<String>),
    /// 委托事件, 发往: broker
    Entrust(Entrust),
    /// 券商推送/同步, 发出: broker
    Broker(BrokerEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum BrokerEvent {
    /// 委托结果，成交单，成交历史，在内部不使用
    Entrust(Vec<Entrust>),
    /// 总资金，可用资金，持仓市值
    FundSync((f32, f32, f32)),
    /// 持仓头寸
    Position(Vec<Position>),
}
