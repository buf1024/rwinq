use crate::{Bar, BarFreq};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// 股票基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    /// 代码
    pub code: String,
    /// 简称
    pub name: String,
    /// 所在板块
    pub block: String,
    /// 是否融资融券
    pub is_margin: bool,
    /// 上市日期
    pub listing_date: NaiveDate,
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
    /// 简称
    pub name: String,
    /// 市值日期
    pub trade_date: NaiveDate,
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

/// 行业基本信息
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
    /// 季度
    pub season: u16,
    /// 季度时间
    pub season_date: NaiveDate,
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
    pub trade_date: NaiveDate,
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

/// 实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockRtQuot {
    /// 代码
    pub code: String,
    /// 行情时间
    // #[serde(with = "naive_date_time_format")]
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
    pub last: f32,
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
