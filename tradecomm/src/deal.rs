use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::TradeType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deal {
    /// id
    pub id: String,
    /// 类型
    pub typ: TradeType,
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
    /// 价格
    pub price: f32,
    /// 量
    pub volume: i32,
    /// 盈利
    pub profit: f32,
    /// 手续费
    pub fee: f32,
    /// 描述
    pub desc: String,
}

impl Default for Deal {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_simple().to_string(),
            typ: Default::default(),
            code: Default::default(),
            name: Default::default(),
            time: Local::now().naive_local(),
            price: Default::default(),
            volume: Default::default(),
            profit: Default::default(),
            fee: Default::default(),
            desc: Default::default(),
        }
    }
}
