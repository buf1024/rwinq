//! 公共数据

use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 交易日历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDate {
    /// 交易日
    pub trade_date: i32,
}

/// K线基本信息（可表示分钟线，日线，周线，年线等）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bar {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 交易时间
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub trade_date: NaiveDateTime,
    /// 开盘价
    pub open: f32,
    /// 收盘价
    pub close: f32,
    /// 最高价
    pub high: f32,
    /// 最低价
    pub low: f32,
    /// 成交量(手)
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

/// k线频率
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BarFreq {
    /// 1 分钟
    Min1 = 1,
    /// 5 分钟
    Min5 = 5,
    /// 15 分钟
    Min15 = 15,
    /// 30 分钟
    Min30 = 30,
    /// 60 分钟
    Min60 = 60,
    /// 日频
    Daily = 101,
    /// 周频
    Weekly = 102,
    /// 月频
    Monthly = 103,
    /// 宽松版日频，当前交易交易时间，如果少于15点交易时间时
    /// 取当前最新价，如果大于15点，和日频一样
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

/// 实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotXq {
    /// 代码
    pub code: String,
    /// 行情时间
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub time: NaiveDateTime,
    /// 昨收价
    pub last_close: f32,
    /// 开盘价
    pub open: f32,
    /// 最高价
    pub high: f32,
    /// 最低价
    pub low: f32,
    /// 最后一口价（最新价）
    pub now: f32,
    /// 涨跌额
    pub chg: f32,
    /// 涨幅
    pub chg_pct: f32,
    /// 成交量
    pub volume: i64,
    /// 成交额
    pub amount: f64,
    /// 换手率
    pub turnover: f32,
    /// 总市值
    pub total_value: f64,
    /// 流通市值
    pub currency_value: f64,
    /// 是否交易
    pub is_trading: bool,
}

pub type RtQuotXq = HashMap<String, QuotXq>;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct QuotSn {
    /// 代码
    pub code: String,
    /// 名称
    pub name: String,
    /// 开盘价
    pub open: f64,
    /// 昨收价
    pub last_close: f64,
    /// 最后一口价（最新价）
    pub now: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 买价
    pub buy: f64,
    /// 卖价
    pub sell: f64,
    /// 成交量
    pub volume: u64,
    /// 成交额
    pub amount: f64,
    /// 买单
    pub bid: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    /// 卖单
    pub ask: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    /// 行情时间
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub time: NaiveDateTime,
}

pub type RtQuotSn = HashMap<String, QuotSn>;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Quot {
    /// 代码
    pub code: String,
    /// 名称
    pub name: String,
    /// 开盘价
    pub open: f64,
    /// 昨收价
    pub last_close: f64,
    /// 最后一口价（最新价）
    pub now: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 买价
    pub buy: f64,
    /// 卖价
    pub sell: f64,
    /// 成交量
    pub volume: u64,
    /// 成交额
    pub amount: f64,
    /// 买单
    pub bid: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    /// 卖单
    pub ask: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    /// 行情时间
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub time: NaiveDateTime,

    /// 涨跌额
    pub chg: f32,
    /// 涨幅
    pub chg_pct: f32,
    /// 换手率
    pub turnover: f32,
    /// 总市值
    pub total_value: f64,
    /// 流通市值
    pub currency_value: f64,
    /// 是否交易
    pub is_trading: bool,
}

pub type RtQuot = HashMap<String, Quot>;
