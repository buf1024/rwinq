//! etf场内基金基本数据

use crate::{Bar, BarFreq};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// etf基金基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundInfo {
    /// 代码
    pub code: String,
    /// etf基金简称
    pub name: String,
}

/// 基金净值基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundNet {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 净值日期
    pub trade_date: NaiveDateTime,
    /// 净值
    pub net: f32,
    /// 累计净值
    pub net_acc: f32,
    /// 日增长率
    pub chg_pct: f32,
    /// 申购状态
    pub apply_status: String,
    /// 赎回状态
    pub redeem_status: String,
}

/// 基金k线基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundBar {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 频率
    pub freq: BarFreq,
    /// bar数据
    pub bars: Option<Vec<Bar>>,
}
