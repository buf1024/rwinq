use serde::{Deserialize, Serialize};

use crate::{Signal, TradeType, Uuid, TradeTime};

/// 委托单状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntrustStatus {
    Init = 0,
    Commit = 1,
    Deal = 2,
    PartDeal = 3,
    Cancel = 4,
}
impl Default for EntrustStatus {
    fn default() -> Self {
        EntrustStatus::Cancel
    }
}

/// 委托单
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Entrust {
    /// id
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub id: Uuid,
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
    /// broker对应的委托id
    pub broker_entrust_id: Option<String>,
    /// 委托类型
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub typ: TradeType,
    /// 委托状态
    pub status: EntrustStatus,
    /// 委托价格
    pub price: f32,
    /// 委托量
    pub volume: u32,
    /// 已成交量
    pub volume_deal: u32,
    /// 已取消量
    pub volume_cancel: u32,
    /// 触发委托的信号id
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub signal_id: Uuid,
    /// 描述
    pub desc: String,
}

impl From<&Signal> for Entrust {
    fn from(signal: &Signal) -> Self {
        Self {
            id: Default::default(),
            name: signal.name.clone(),
            code: signal.code.clone(),
            time: Default::default(),
            typ: signal.typ,
            status: EntrustStatus::Init,
            price: signal.price,
            volume: signal.volume,
            volume_deal: 0,
            volume_cancel: 0,
            desc: signal.desc.clone(),
            broker_entrust_id: None,
            signal_id: signal.id.clone(),
        }
    }
}
