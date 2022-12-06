use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 方便数据库存储
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDate {
    /// 交易日
    pub trade_date: i32,
}

/// 可日线基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 交易日
    pub trade_date: NaiveDateTime,
    /// 开盘价
    pub open: f32,
    /// 收盘价
    pub close: f32,
    /// 最高价
    pub high: f32,
    /// 最低价
    pub low: f32,
    /// 成交量(股)
    pub volume: u64,
    /// 成交额(元)
    pub amount: f64,
    /// 换手率(百分比)
    pub turnover: f32,
    /// 涨跌比(百分比)
    pub chg_pct: f32,
    /// 成交量变更(百分比)
    pub volume_chg_pct: f32,
    /// 成交额变更(百分比)
    pub amount_chg_pct: f32,
    /// 后复权因子
    /// 量化时一般采用后复权，前复权可能出现股价位负数，一般不使用
    pub hfq_factor: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BarFreq {
    Min1 = 1,
    Min5 = 5,
    Min15 = 15,
    Min30 = 30,
    Min60 = 60,
    Daily = 101,
    Weekly = 102,
    Monthly = 103,
    LooseDaily = 1010,
}

impl From<i32> for BarFreq {
    fn from(v: i32) -> Self {
        match v {
            1 => BarFreq::Min1,
            5 => BarFreq::Min5,
            15 => BarFreq::Min15,
            30 => BarFreq::Min30,
            60 => BarFreq::Min60,
            101 => BarFreq::Daily,
            1010 => BarFreq::LooseDaily,
            102 => BarFreq::Weekly,
            103 => BarFreq::Monthly,
            _ => BarFreq::Daily,
        }
    }
}
