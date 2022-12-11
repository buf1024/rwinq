//! 获取etf基金接口
//!
//! 如有其他实现方式，实现该trait即可。

#![allow(unused_variables)]

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_common::{BarFreq, FundBar, FundInfo, FundNet};

/// 获取etf基金接口
#[async_trait]
pub trait FundFetch: Sync + Send {
    /// etf基金基本信息
    async fn fetch_fund_info(&self) -> Result<Vec<FundInfo>> {
        Err(Error::NotImpl("fetch_fund_info".to_string()))
    }
    /// etf基金净值
    async fn fetch_fund_net(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<FundNet>> {
        Err(Error::NotImpl("fetch_fund_net".to_string()))
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
        Err(Error::NotImpl("fetch_fund_bar".to_string()))
    }
}
