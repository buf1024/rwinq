use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 信号类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    Buy = 0,
    Sell = 1,
    Cancel = 2,
}

/// 信号源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalSource {
    Risk = 0,
    Strategy = 1,
    Broker = 2,
    Manual = 3,
}

/// 信号源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// id
    pub id: String,
    /// 信号类型
    pub typ: SignalType,
    /// 信号源
    pub source: SignalSource,
    /// 代码
    pub code: String,
    /// 名称
    pub name: String,
    /// 时间
    #[serde(
        serialize_with = "rwqcmm::naive_dt_serialize",
        deserialize_with = "rwqcmm::naive_dt_deserialize"
    )]
    pub time: NaiveDateTime,
    /// 委托价格
    pub price: f32,
    /// 委托量
    pub volume: i32,
    /// 描述
    pub desc: String,
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_simple().to_string(),
            typ: SignalType::Sell,
            source: SignalSource::Manual,
            code: Default::default(),
            name: Default::default(),
            time: Local::now().naive_local(),
            price: Default::default(),
            volume: Default::default(),
            desc: Default::default(),
        }
    }
}
