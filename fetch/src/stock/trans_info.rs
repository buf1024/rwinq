use serde::{Deserialize, Serialize};

/// stock_info bj
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchSHStockInfo<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "pageHelp"))]
    pub page_help: ExchSHStockInfoPageHelp<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchSHStockInfoPageHelp<'a> {
    #[serde(borrow)]
    pub data: Vec<ExchSHStockInfoData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchSHStockInfoData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "COMPANY_ABBR"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "A_STOCK_CODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "DELIST_DATE"))]
    pub de_list: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "LIST_DATE"))]
    pub list_date: &'a str,
}

/// stock_info bj
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchBJStockInfo<'a> {
    #[serde(borrow)]
    pub content: Vec<ExchBJStockInfoData<'a>>,

    #[serde(rename(deserialize = "totalPages"))]
    pub total_page: isize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExchBJStockInfoData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "xxzqjc"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "xxzqdm"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "fxssrq"))]
    pub list_date: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockInfoMargin<'a> {
    #[serde(borrow)]
    pub data: EastStockInfoMarginData<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockInfoMarginData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockInfoMarginDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockInfoMarginDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,
}

// stock index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndex<'a> {
    #[serde(borrow)]
    pub data: Option<EastStockIndexData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndexData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockIndexDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum EastFloatString<'a, T: num_traits::Float + Default> {
    Float(T),
    String(&'a str),
}

impl<'a, T: num_traits::Float + Default> Default for EastFloatString<'a, T> {
    fn default() -> Self {
        Self::Float(T::default())
    }
}

impl<'a, T: num_traits::Float + Default> EastFloatString<'a, T> {
    pub(crate) fn unwrap(&self) -> T {
        match self {
            EastFloatString::Float(v) => *v,
            EastFloatString::String(_) => T::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndexDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f14"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f2"))]
    pub price: EastFloatString<'a, f32>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f9"))]
    pub pe: EastFloatString<'a, f32>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f23"))]
    pub pb: EastFloatString<'a, f32>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f20"))]
    pub total_value: EastFloatString<'a, f64>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f21"))]
    pub currency_value: EastFloatString<'a, f64>,
}

// stock index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndustry<'a> {
    #[serde(borrow)]
    pub data: EastStockIndustryData<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndustryData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockIndustryDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockIndustryDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f14"))]
    pub name: &'a str,
}

/// stock_yjbb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockYJBB<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockYJBBResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockYJBBResult<'a> {
    pub pages: usize,
    #[serde(borrow)]
    pub data: Vec<EastStockYJBBData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockYJBBData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_CODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_NAME_ABBR"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "REPORTDATE"))]
    pub season_date: &'a str,

    #[serde(rename(deserialize = "BASIC_EPS"))]
    pub mg_sy: Option<f32>,

    #[serde(rename(deserialize = "TOTAL_OPERATE_INCOME"))]
    pub yysr: Option<f64>,

    #[serde(rename(deserialize = "YSTZ"))]
    pub yysr_tbzz: Option<f32>,

    #[serde(rename(deserialize = "YSHZ"))]
    pub yysr_jdhbzz: Option<f32>,

    #[serde(rename(deserialize = "PARENT_NETPROFIT"))]
    pub jlr: Option<f64>,

    #[serde(rename(deserialize = "SJLTZ"))]
    pub jlr_tbzz: Option<f32>,

    #[serde(rename(deserialize = "SJLHZ"))]
    pub jlr_jdhbzz: Option<f32>,

    #[serde(rename(deserialize = "BPS"))]
    pub mg_jzc: Option<f32>,

    #[serde(rename(deserialize = "WEIGHTAVG_ROE"))]
    pub jzc_syl: Option<f32>,

    #[serde(rename(deserialize = "MGJYXJJE"))]
    pub mg_jy_xjl: Option<f64>,

    #[serde(rename(deserialize = "XSMLL"))]
    pub xs_mll: Option<f32>,
}

/// stock_margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMargin<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockMarginResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMarginResult<'a> {
    pub pages: usize,
    #[serde(borrow)]
    pub data: Vec<EastStockMarginData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMarginData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "SCODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SECNAME"))]
    pub name: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "DATE"))]
    pub trade_date: &'a str,

    /// 收盘价(元)(SPJ)
    #[serde(rename(deserialize = "SPJ"))]
    pub close: Option<f32>,
    /// 涨跌幅(%)(ZDF):
    #[serde(rename(deserialize = "ZDF"))]
    pub chg_pct: Option<f32>,
    /// 融资: 余额(元)(RZYE)
    #[serde(rename(deserialize = "RZYE"))]
    pub rz_ye: Option<f32>,
    /// 余额占流通市值比(%)(RZYEZB)
    #[serde(rename(deserialize = "RZYEZB"))]
    pub rz_ye_zb: Option<f32>,
    /// 买入额(元)(RZMRE)
    #[serde(rename(deserialize = "RZMRE"))]
    pub rz_mre: Option<f64>,
    ///	偿还额(元)(RZCHE)
    #[serde(rename(deserialize = "RZCHE"))]
    pub rz_che: Option<f64>,
    ///	净买入(元)(RZJME)
    #[serde(rename(deserialize = "RZJME"))]
    pub rz_jme: Option<f64>,
    /// 融券: 余额(元)(RQYE)
    #[serde(rename(deserialize = "RQYE"))]
    pub rq_ye: Option<f64>,
    ///	余量(股)(RQYL)
    #[serde(rename(deserialize = "RQYL"))]
    pub rq_yl: Option<i32>,
    /// 卖出量(股)(RQMCL)
    #[serde(rename(deserialize = "RQMCL"))]
    pub rq_mcl: Option<i32>,
    ///	偿还量(股)(RQCHL)
    #[serde(rename(deserialize = "RQCHL"))]
    pub rq_chl: Option<i32>,
    /// 净卖出(股)(RQJMG)
    #[serde(rename(deserialize = "RQJMG"))]
    pub rq_jmg: Option<i32>,
    /// 融资融券余额(元)(RZRQYE)
    #[serde(rename(deserialize = "RZRQYE"))]
    pub rz_rq_ye: Option<f64>,
    /// 融资融券余额差值(元)(RZRQYECZ)
    #[serde(rename(deserialize = "RZRQYECZ"))]
    pub rz_rq_ye_cz: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockHotRankResult<'a> {
    // #[serde(borrow)]
    // #[serde(rename(deserialize = "globalId"))]
    // pub global_id: &'a str,
    // #[serde(borrow)]
    // pub message: &'a str,
    // pub status: i32,
    // pub code: i32,
    #[serde(borrow)]
    pub data: EastStockHotRankData<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockHotRankData<'a> {
    // #[serde(borrow)]
    // #[serde(rename(deserialize = "marketType"))]
    // pub market_type: &'a str,
    #[serde(rename(deserialize = "marketAllCount"))]
    pub market_all_count: i32,

    #[serde(borrow)]
    #[serde(rename(deserialize = "calcTime"))]
    pub calc_time: &'a str,

    // #[serde(borrow)]
    // #[serde(rename(deserialize = "innerCode"))]
    // pub inner_code: &'a str,

    // #[serde(borrow)]
    // #[serde(rename(deserialize = "srcSecurityCode"))]
    // pub src_security_code: &'a str,
    pub rank: i32,

    #[serde(rename(deserialize = "rankChange"))]
    pub rank_chg: i32,
    // #[serde(rename(deserialize = "hisRankChange"))]
    // pub his_rank_change: i32,

    // #[serde(rename(deserialize = "hisRankChange_rank"))]
    // pub his_rank_change_rank: i32,

    // pub flag: i32,
}

// 上深京行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockQuot<'a> {
    #[serde(borrow)]
    pub data: Option<EastStockQuotData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockQuotData<'a> {
    pub total: usize,
    #[serde(borrow)]
    pub diff: Vec<EastStockQuotDataDetail<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockQuotDataDetail<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "f12"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "f14"))]
    pub name: &'a str,

    #[serde(rename(deserialize = "f2"))]
    pub price: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f3"))]
    pub chg_pct: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f4"))]
    pub chg: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f5"))]
    pub volume: EastFloatString<'a, f64>,

    #[serde(rename(deserialize = "f6"))]
    pub amount: EastFloatString<'a, f64>,

    #[serde(rename(deserialize = "f8"))]
    pub turnover: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f9"))]
    pub pe: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f10"))]
    pub vol_ratio: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f15"))]
    pub high: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f16"))]
    pub low: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f17"))]
    pub open: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f18"))]
    pub last_close: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "f20"))]
    pub total_value: EastFloatString<'a, f64>,

    #[serde(rename(deserialize = "f21"))]
    pub currency_value: EastFloatString<'a, f64>,

    #[serde(rename(deserialize = "f22"))]
    pub rise_speed: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "23"))]
    pub pb: Option<EastFloatString<'a, f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockComment<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockCommentResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentResult<'a> {
    pub pages: i32,
    #[serde(borrow)]
    pub data: Vec<EastStockCommentData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_CODE"))]
    pub code: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_NAME_ABBR"))]
    pub name: &'a str,

    #[serde(rename(deserialize = "CLOSE_PRICE"))]
    pub close: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "CHANGE_RATE"))]
    pub chg_pct: EastFloatString<'a, f32>,

    #[serde(rename(deserialize = "TURNOVERRATE"))]
    pub turnover: f32,

    #[serde(rename(deserialize = "PE_DYNAMIC"))]
    pub pe: f32,

    #[serde(rename(deserialize = "PRIME_COST"))]
    pub cost: f32,

    #[serde(rename(deserialize = "ORG_PARTICIPATE"))]
    pub engage: f32,

    #[serde(rename(deserialize = "TOTALSCORE"))]
    pub score: Option<f32>,

    #[serde(rename(deserialize = "RANK"))]
    pub rank: Option<i32>,

    #[serde(rename(deserialize = "RANK_UP"))]
    pub rank_chg: Option<i32>,

    #[serde(rename(deserialize = "FOCUS"))]
    pub attention: Option<f32>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "TRADE_DATE"))]
    pub trade_date: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentScore<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockCommentScoreResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentScoreResult<'a> {
    pub pages: i32,
    #[serde(borrow)]
    pub data: Vec<EastStockCommentScoreData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentScoreData<'a> {
    #[serde(rename(deserialize = "TOTAL_SCORE"))]
    pub score: f32,

    #[serde(borrow)]
    #[serde(rename(deserialize = "DIAGNOSE_DATE"))]
    pub trade_date: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentAttention<'a> {
    #[serde(borrow)]
    pub result: Option<EastStockCommentAttentionResult<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentAttentionResult<'a> {
    pub pages: i32,
    #[serde(borrow)]
    pub data: Vec<EastStockCommentAttentionData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockCommentAttentionData<'a> {
    #[serde(rename(deserialize = "MARKET_FOCUS"))]
    pub attention: f32,

    #[serde(rename(deserialize = "MARKET_FOCUS_RANK"))]
    pub rank: Option<i32>,

    #[serde(rename(deserialize = "MARKET_FOCUS_CHANGE"))]
    pub rank_chg: Option<i32>,

    #[serde(borrow)]
    #[serde(rename(deserialize = "TRADE_DATE"))]
    pub trade_date: &'a str,
}
