use serde::{Deserialize, Serialize};

/// fund_net
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastFundNet<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "Data"))]
    pub data: Option<EastFundNetData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastFundNetData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "LSJZList"))]
    pub list: Vec<EastStockMarginDataItem<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastStockMarginDataItem<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "FSRQ"))]
    pub trade_date: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "DWJZ"))]
    pub net: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "LJJZ"))]
    pub net_acc: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "JZZZL"))]
    pub chg_pct: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SGZT"))]
    pub apply_status: &'a str,

    #[serde(borrow)]
    #[serde(rename(deserialize = "SHZT"))]
    pub redeem_status: &'a str,
}
