use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastBar<'a> {
    #[serde(borrow)]
    pub data: Option<EastBarData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EastBarData<'a> {
    #[serde(borrow)]
    pub code: &'a str,
    #[serde(borrow)]
    pub name: &'a str,
    #[serde(borrow)]
    pub klines: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XueQiuBar<'a> {
    #[serde(borrow)]
    pub data: Option<XueQiuBarData<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XueQiuBarData<'a> {
    #[serde(borrow)]
    #[serde(rename(deserialize = "symbol"))]
    pub code: &'a str,
    // ["timestamp","volume","open","high","low","close","chg","percent","turnoverrate","amount","volume_post","amount_post"]
    pub item: Vec<XueQiuBarDataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XueQiuBarDataItem(
    // ["timestamp","volume","open","high","low","close","chg","percent","turnoverrate","amount","volume_post","amount_post"]
    pub i64,
    pub Option<u64>,
    pub Option<f32>,
    pub Option<f32>,
    pub Option<f32>,
    pub Option<f32>,
    pub Option<f32>,
    pub Option<f32>,
    pub Option<f32>,
    pub Option<f64>,
    pub Option<i64>,
    pub Option<f64>,
);
