#![allow(unused_variables)]

use crate::{Error, Result};
use chrono::NaiveDate;
use hiq_common::{BarFreq, FundBar, FundInfo, FundNet};
use async_trait::async_trait;

#[async_trait]
pub trait FundFetch: Sync + Send {
    /// etf基金基本信息
    async fn fetch_fund_info(&self) -> Result<Vec<FundInfo>>
    {
        Err(Error::NotImpl("fetch_fund_info"))
    }
    /// etf基金净值
    async fn fetch_fund_net(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<FundNet>> {
        Err(Error::NotImpl("fetch_fund_net"))
    }
    /// etf基金k线数据
    async fn fetch_fund_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<FundBar> {
        Err(Error::NotImpl("fetch_fund_bar"))
    }
}
