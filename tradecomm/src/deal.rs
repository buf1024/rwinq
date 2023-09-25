use serde::{Deserialize, Serialize};

use crate::{Entrust, TradeTime, TradeType, Uuid};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Deal {
    /// id
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub id: Uuid,
    /// 类型
    pub typ: TradeType,
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
    /// 价格
    pub price: f32,
    /// 量
    pub volume: u32,
    /// 盈利
    pub profit: f32,
    /// 手续费
    pub fee: f32,
    /// 描述
    pub desc: String,
    /// 委托ID
    pub entrust_id: Uuid,
}

impl From<&Entrust> for Deal {
    fn from(entrust: &Entrust) -> Self {
        Self {
            id: Default::default(),
            entrust_id: entrust.id.clone(),
            name: entrust.name.clone(),
            code: entrust.code.clone(),
            time: Default::default(),
            typ: entrust.typ.clone(),
            price: entrust.price,
            volume: entrust.volume_deal,
            profit: 0.0,
            fee: 0.0,
            desc: entrust.desc.clone(),
        }
    }
}
