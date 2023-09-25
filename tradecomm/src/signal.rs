use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{TradeTime, TradeType, Uuid};

/// 信号源
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    Risk(String),
    Strategy(String),
    Broker(String),
    Manual(String),
}
impl Default for SignalSource {
    fn default() -> Self {
        SignalSource::Manual("Unknown".to_string())
    }
}

impl Display for SignalSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SignalSource::Risk(v) => format!("Risk: {}", v),
            SignalSource::Strategy(v) => format!("Strategy: {}", v),
            SignalSource::Broker(v) => format!("Broker: {}", v),
            SignalSource::Manual(v) => format!("Manual: {}", v),
        };
        write!(f, "{}", s)
    }
}

/// 信号源
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Signal {
    /// id
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub id: Uuid,
    /// 信号类型
    pub typ: TradeType,
    /// 信号源
    pub source: SignalSource,
    /// 代码
    pub code: String,
    /// 名称
    pub name: String,
    /// 时间
    #[serde(
        serialize_with = "crate::trade_time_serialize",
        deserialize_with = "crate::trade_time_deserialize"
    )]
    pub time: TradeTime,
    /// 委托价格
    pub price: f32,
    /// 委托量
    pub volume: u32,
    /// 描述
    pub desc: String,
}
