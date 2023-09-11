//! 公共基本数据类型
use std::path::PathBuf;

use rwqfetch::{
    Bar, BondInfo, FundInfo, FundNet, StockConcept, StockConceptDetail, StockIndex, StockIndustry,
    StockIndustryDetail, StockInfo, StockMargin, StockYJBB, TradeDate,
};

use crate::Error;

/// 目的数据源
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncDest {
    File(PathBuf),
    MongoDB(String),
    MySQL(String),
    ClickHouse(String),
}

/// 转换为`SyncDest`  
/// 格式为(file, path), (mongodb, url), (mysql, url), (clickhouse, url)
impl TryFrom<(String, String)> for SyncDest {
    type Error = Error;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let typ = value.0.to_lowercase();
        let val = value.1;
        match typ.as_str() {
            "file" => Ok(SyncDest::File(PathBuf::from(val))),
            "mongodb" => Ok(SyncDest::MongoDB(val)),
            "mysql" => Ok(SyncDest::MySQL(val)),
            "clickhouse" => Ok(SyncDest::ClickHouse(val)),
            _ => Err(Error::Custom(format!("Invalid SyncDest: {}", typ.as_str()))),
        }
    }
}

/// 目标数据源类型
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SyncDestType {
    File = 1,
    MongoDB,
    MySQL,
    ClickHouse,
}

impl TryFrom<i32> for SyncDestType {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(SyncDestType::File),
            2 => Ok(SyncDestType::MongoDB),
            3 => Ok(SyncDestType::MySQL),
            4 => Ok(SyncDestType::ClickHouse),
            _ => Err(Error::Custom(format!("Invalid SyncDestType: {}", v))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyncData {
    TradeDate(Vec<TradeDate>),
    // stock
    IndexInfo(Vec<StockInfo>),
    IndexBar(Vec<Bar>),
    StockInfo(Vec<StockInfo>),
    StockBar(Vec<Bar>),
    StockIndex(Vec<StockIndex>),
    StockIndustry(Vec<StockIndustry>),
    StockIndustryDetail(Vec<StockIndustryDetail>),
    StockIndustryBar(Vec<Bar>),
    StockConcept(Vec<StockConcept>),
    StockConceptDetail(Vec<StockConceptDetail>),
    StockConceptBar(Vec<Bar>),
    StockYJBB(Vec<StockYJBB>),
    StockMargin(Vec<StockMargin>),

    // fund
    FundInfo(Vec<FundInfo>),
    FundNet(Vec<FundNet>),
    FundBar(Vec<Bar>),

    // bond
    BondInfo(Vec<BondInfo>),
    BondBar(Vec<Bar>),

    // tag
    Done,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SyncDataType {
    TradeDate = 1,
    // stock
    IndexInfo,
    IndexBar,
    StockInfo,
    StockBar,
    StockIndex,
    StockIndustry,
    StockIndustryDetail,
    StockIndustryBar,
    StockConcept,
    StockConceptDetail,
    StockConceptBar,
    StockYJBB,
    StockMargin,

    // fund
    FundInfo,
    FundNet,
    FundBar,

    // bond
    BondInfo,
    BondBar,
}

impl TryFrom<i32> for SyncDataType {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(SyncDataType::TradeDate),
            // stock
            2 => Ok(SyncDataType::IndexInfo),
            3 => Ok(SyncDataType::IndexBar),
            4 => Ok(SyncDataType::StockInfo),
            5 => Ok(SyncDataType::StockBar),
            6 => Ok(SyncDataType::StockIndex),
            7 => Ok(SyncDataType::StockIndustry),
            8 => Ok(SyncDataType::StockIndustryDetail),
            9 => Ok(SyncDataType::StockIndustryBar),
            10 => Ok(SyncDataType::StockConcept),
            11 => Ok(SyncDataType::StockConceptDetail),
            12 => Ok(SyncDataType::StockConceptBar),
            13 => Ok(SyncDataType::StockYJBB),
            14 => Ok(SyncDataType::StockMargin),

            // fund
            15 => Ok(SyncDataType::FundInfo),
            16 => Ok(SyncDataType::FundNet),
            17 => Ok(SyncDataType::FundBar),

            // bond
            18 => Ok(SyncDataType::BondInfo),
            19 => Ok(SyncDataType::BondBar),
            _ => Err(Error::Custom(format!("Invalid SyncDataType: {}", v))),
        }
    }
}

impl TryFrom<&str> for SyncDataType {
    type Error = Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let v = v.to_lowercase();

        match v.as_str() {
            "trade_date" => Ok(SyncDataType::TradeDate),
            // stock
            "index_info" => Ok(SyncDataType::IndexInfo),
            "index_daily" => Ok(SyncDataType::IndexBar),
            "stock_info" => Ok(SyncDataType::StockInfo),
            "stock_daily" => Ok(SyncDataType::StockBar),
            "stock_index" => Ok(SyncDataType::StockIndex),
            "stock_industry" => Ok(SyncDataType::StockIndustry),
            "stock_industry_detail" => Ok(SyncDataType::StockIndustryDetail),
            "stock_industry_daily" => Ok(SyncDataType::StockIndustryBar),
            "stock_concept" => Ok(SyncDataType::StockConcept),
            "stock_concept_detail" => Ok(SyncDataType::StockConceptDetail),
            "stock_concept_daily" => Ok(SyncDataType::StockConceptBar),
            "stock_yjbb" => Ok(SyncDataType::StockYJBB),
            "stock_margin" => Ok(SyncDataType::StockMargin),

            // fund
            "fund_info" => Ok(SyncDataType::FundInfo),
            "fund_net" => Ok(SyncDataType::FundNet),
            "fund_daily" => Ok(SyncDataType::FundBar),

            // bond
            "bond_info" => Ok(SyncDataType::BondInfo),
            "bond_daily" => Ok(SyncDataType::BondBar),
            _ => Err(Error::Custom(format!(
                "Invalid SyncDataType: {}",
                v.as_str()
            ))),
        }
    }
}
