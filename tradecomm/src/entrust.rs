use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::TradeType;


/// 委托单状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntrustStatus {
    Init = 0,
    Commit = 1,
    Deal = 2,
    PartDeal = 3,
    Cancel = 4,
}

/// 委托单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entrust {
    /// id
    pub id: String,
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
    pub volume: i32,
    /// 已成交量
    pub volume_deal: i32,
    /// 已取消量
    pub volume_cancel: i32,
    /// 触发委托的信号id
    pub signal_id: String,
    /// 描述
    pub desc: String,
}

impl Default for Entrust {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_simple().to_string(),
            code: Default::default(),
            name: Default::default(),
            time: Local::now().naive_local(),
            broker_entrust_id: Default::default(),
            typ: Default::default(),
            status: EntrustStatus::Cancel,
            price: Default::default(),
            volume: Default::default(),
            volume_deal: Default::default(),
            volume_cancel: Default::default(),
            signal_id: Default::default(),
            desc: Default::default(),
        }
    }
}
