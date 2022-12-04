use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastBondInfo<'a> {
    #[serde(borrow)]
    pub result: EastBondInfoResult<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastBondInfoResult<'a> {
    pub pages: i64,
    #[serde(borrow)]
    pub data: Vec<EastBondInfoData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastBondInfoData<'a> {
    /// 债券代码
    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_CODE"))]
    pub code: &'a str,
    /// 债券简称
    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_NAME_ABBR"))]
    pub name: &'a str,

    /// 正股代码
    #[serde(borrow)]
    #[serde(rename(deserialize = "CONVERT_STOCK_CODE"))]
    pub stock_code: &'a str,
    /// 正股简称
    #[serde(borrow)]
    #[serde(rename(deserialize = "SECURITY_SHORT_NAME"))]
    pub stock_name: &'a str,

    /// 退市时间
    #[serde(borrow)]
    #[serde(rename(deserialize = "DELIST_DATE"))]
    pub delist_date: Option<&'a str>,

    /// 上市时间
    #[serde(borrow)]
    #[serde(rename(deserialize = "LISTING_DATE"))]
    pub listing_date: Option<&'a str>,
}
