//! 可转债数据

use crate::{Bar, BarFreq};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 可转债基本信息（已经确定上市交易的可转债）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondInfo {
    /// 可转债代码
    pub code: String,
    /// 可转债简称
    pub name: String,
    /// 正股代码
    pub stock_code: String,
    /// 正股简称
    pub stock_name: String,
    /// 上市时间
    /// 1. 没上市的可转债不会返回
    /// 2. 上市时间大于当前交易日，仅代表已经确认上市时间
    #[serde(serialize_with = "crate::naive_dt_serialize", deserialize_with="crate::naive_dt_deserialize")]
    pub listing_date: NaiveDateTime,
    /// 是否已经退市 0, 1
    pub is_delist: u8,
}

/// 可转债k线基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondBar {
    /// 可转债代码
    pub code: String,
    /// 可转债简称
    pub name: String,
    /// 正股代码
    pub stock_code: String,
    /// 正股简称
    pub stock_name: String,
    /// 频率
    pub freq: BarFreq,
    /// bar数据
    pub bars: Option<Vec<Bar>>,
}
