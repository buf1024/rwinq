#![allow(unused_variables)]

use crate::{Error, Result};
use chrono::NaiveDate;
use hiq_common::stock::*;
use hiq_common::BarFreq;
use std::collections::{HashMap, HashSet};
use async_trait::async_trait;

#[async_trait]
pub trait StockFetch: Sync + Send {
    /// 股票大盘指数（列举几个而已）
    async fn fetch_index_info(&self) -> Result<Vec<StockInfo>> {
        Err(Error::NotImpl("fetch_index_info"))
    }
    /// 指数k线数据
    async fn fetch_index_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<StockBar> {
        Err(Error::NotImpl("fetch_index_bar"))
    }
    /// 获取股票基本信息
    async fn fetch_stock_info(&self) -> Result<Vec<StockInfo>> {
        Err(Error::NotImpl("fetch_stock_info"))
    }
    /// 获取融资融券股票代码
    async fn fetch_stock_is_margin(&self) -> Result<HashSet<String>> {
        Err(Error::NotImpl("fetch_stock_is_margin"))
    }
    /// 股票/指数k线数据
    async fn fetch_stock_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<StockBar> {
        Err(Error::NotImpl("fetch_stock_bar"))
    }

    /// 股票最新指标
    async fn fetch_stock_index(&self, date: Option<NaiveDate>) -> Result<HashMap<String, StockIndex>> {
        Err(Error::NotImpl("fetch_stock_index"))
    }

    /// 股票行业
    async fn fetch_stock_industry(&self) -> Result<Vec<StockIndustry>> {
        Err(Error::NotImpl("fetch_stock_industry"))
    }

    /// 股票行业详情
    async fn fetch_stock_industry_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> Result<Vec<StockIndustryDetail>> {
        Err(Error::NotImpl("fetch_stock_industry_detail"))
    }

    /// 股票行业k线数据，只有日频率
    async fn fetch_stock_industry_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<StockIndustryBar> {
        Err(Error::NotImpl("fetch_stock_industry_daily"))
    }

    /// 股票概念
    async fn fetch_stock_concept(&self) -> Result<Vec<StockConcept>> {
        Err(Error::NotImpl("fetch_stock_concept"))
    }

    /// 股票概念详情
    async fn fetch_stock_concept_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> Result<Vec<StockConceptDetail>> {
        Err(Error::NotImpl("fetch_stock_concept_detail"))
    }

    /// 股票概念k线数据
    async fn fetch_stock_concept_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<StockConceptBar> {
        Err(Error::NotImpl("fetch_stock_concept_daily"))
    }

    /// 股票业绩报表
    async fn fetch_stock_yjbb(&self, year: u16, season: u16) -> Result<Vec<StockYJBB>> {
        Err(Error::NotImpl("fetch_stock_yjbb"))
    }
    /// 融资融券
    async fn fetch_stock_margin(
        &self,
        code: &str,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<StockMargin>> {
        Err(Error::NotImpl("fetch_stock_margin"))
    }
    /// 实时行情
    async fn fetch_stock_rt_quot(&self, code: Vec<&str>) -> Result<HashMap<String, StockRtQuot>> {
        Err(Error::NotImpl("fetch_stock_rt_quot"))
    }
}
