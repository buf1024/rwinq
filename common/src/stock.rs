//! 股票基本数据

use crate::{Bar, BarFreq};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// 股票基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    /// 代码
    pub code: String,
    /// 股票简称
    pub name: String,
    /// 股票所在板块
    pub block: String,
    /// 是否融资融券标的
    pub is_margin: bool,
    /// 上市日期
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub listing_date: NaiveDateTime,
}

/// 股票k线基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockBar {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 频率
    pub freq: BarFreq,
    /// bar数据
    pub bars: Option<Vec<Bar>>,
}

/// 股票指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndex {
    /// 代码
    pub code: String,
    /// 股票简称
    pub name: String,
    /// 市值日期
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub trade_date: NaiveDateTime,
    /// 股价
    pub price: f32,
    /// pe
    pub pe: f32,
    /// pb
    pub pb: f32,
    /// 总市值
    pub total_value: f64,
    /// 流通市值
    pub currency_value: f64,
}

/// 股票行业基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndustry {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
}

/// 行业股票信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndustryDetail {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 股票代码
    pub stock_code: String,
    /// 股票名称
    pub stock_name: String,
}

/// 行业k线基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndustryBar {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 频率
    pub freq: BarFreq,
    /// bar数据
    pub bars: Option<Vec<Bar>>,
}

/// 概念基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockConcept {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
}

/// 概念股票信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockConceptDetail {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 股票代码
    pub stock_code: String,
    /// 股票名称
    pub stock_name: String,
}

/// 概念k线基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockConceptBar {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 频率
    pub freq: BarFreq,
    /// bar数据
    pub bars: Option<Vec<Bar>>,
}

/// 股票业绩报表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockYJBB {
    /// 年份
    pub year: u16,
    /// 季度，1~4
    pub season: u16,
    /// 季度时间
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub season_date: NaiveDateTime,
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 每股收益
    pub mg_sy: f32,
    /// 营业收入
    pub yysr: f64,
    /// 营业收入-同比增长
    pub yysr_tbzz: f32,
    /// 营业收入-季度环比增长
    pub yysr_jdhbzz: f32,
    /// 净利润
    pub jlr: f64,
    /// 净利润-同比增长
    pub jlr_tbzz: f32,
    /// 净利润-季度环比增长
    pub jlr_jdhbzz: f32,
    /// 每股净资产
    pub mg_jzc: f32,
    /// 净资产收益率
    pub jzc_syl: f32,
    /// 每股经营现金流量
    pub mg_jy_xjl: f64,
    /// 销售毛利率
    pub xs_mll: f32,
}

/// 股票融资融券余额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMargin {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 交易日期
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub trade_date: NaiveDateTime,
    /// 收盘价(元)(SPJ)
    pub close: f32,
    /// 涨跌幅(%)(ZDF):
    pub chg_pct: f32,
    /// 融资: 余额(元)(RZYE)
    pub rz_ye: f64,
    /// 余额占流通市值比(%)(RZYEZB)
    pub rz_ye_zb: f32,
    /// 买入额(元)(RZMRE)
    pub rz_mre: f64,
    ///	偿还额(元)(RZCHE)
    pub rz_che: f64,
    ///	净买入(元)(RZJME)
    pub rz_jme: f64,
    /// 融券: 余额(元)(RQYE)
    pub rq_ye: f64,
    ///	余量(股)(RQYL)
    pub rq_yl: i32,
    /// 卖出量(股)(RQMCL)
    pub rq_mcl: i32,
    ///	偿还量(股)(RQCHL)
    pub rq_chl: i32,
    /// 净卖出(股)(RQJMG)
    pub rq_jmg: i32,
    /// 融资融券余额(元)(RZRQYE)
    pub rz_rq_ye: f64,
    /// 融资融券余额差值(元)(RZRQYECZ)
    pub rz_rq_ye_cz: f64,
}

/// 热门股票基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockHotRank {
    /// 代码
    pub code: String,
    /// 股票总数
    pub market_all_count: i32,
    /// 当前排名
    pub rank: i32,
    /// 当前排名变更
    pub rank_chg: i32,
    /// 计算时间
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub calc_time: NaiveDateTime,
}

/// 全量股票实时行情，不要频繁调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockRtQuot {
    /// 股票代码
    pub code: String,
    /// 股票名称
    pub name: String,
    /// 现价
    pub price: f32,
    /// 涨跌幅
    pub chg_pct: f32,
    /// 涨跌额
    pub chg: f32,
    /// 成交量
    pub volume: f64,
    /// 成交额
    pub amount: f64,
    /// 换手率
    pub turnover: f32,
    /// PE
    pub pe: f32,
    /// 涨速
    pub vol_ratio: f32,
    /// 最高价
    pub high: f32,
    /// 最低价
    pub low: f32,
    /// 开盘价
    pub open: f32,
    /// 昨收价
    pub last_close: f32,
    /// 总市值
    pub total_value: f64,
    /// 流通市值
    pub currency_value: f64,
    /// 涨速
    pub rise_speed: f32,
    /// PB
    pub pb: f32,
}

/// 千股千评
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockComment {
    /// 股票代码
    pub code: String,
    /// 股票名称
    pub name: String,
    /// 现价
    pub close: f32,
    /// 涨跌幅
    pub chg_pct: f32,
    /// 换手率
    pub turnover: f32,
    /// PE
    pub pe: f32,
    /// 主力成本
    pub cost: f32,
    /// 主力参与度
    pub engage: f32,
    /// 综合得分
    pub score: f32,
    /// 排名
    pub rank: i32,
    /// 排名变化
    pub rank_chg: i32,
    /// 关注度
    pub attention: f32,
    /// 交易日期
    #[serde(
        serialize_with = "crate::naive_dt_serialize",
        deserialize_with = "crate::naive_dt_deserialize"
    )]
    pub trade_date: NaiveDateTime,
}
