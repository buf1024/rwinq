//! 获取可转债接口
//! 
//! 如有其他实现方式，实现该trait即可。

#![allow(unused_variables)]

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_common::{BarFreq, BondBar, BondInfo};

/// 可转债trait
#[async_trait]
pub trait BondFetch: Sync + Send {
    /// 获取可转债基本信息
    async fn fetch_bond_info(&self) -> Result<Vec<BondInfo>>
    {
        Err(Error::NotImpl("fetch_bond_info".to_string()))
    }
    /// 获取可转债基本
    ///
    /// *code* 可转债代码，其中11开头的为深市，12开头的为沪市。
    /// *freq* 为频次。
    async fn fetch_bond_bar(
        &self,
        code: &str,
        name: &str,
        stock_code: &str,
        stock_name: &str,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> Result<BondBar>
    {
        Err(Error::NotImpl("fetch_bond_bar".to_string()))
    }
}
