use std::path::PathBuf;

use hiq_fetch::{
    Bar, BondInfo, FundInfo, FundNet, StockConcept, StockConceptDetail, StockIndex, StockIndustry,
    StockIndustryDetail, StockInfo, StockMargin, StockYJBB, TradeDate,
};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HiqSyncDest {
    File(PathBuf),
    MongoDB(String),
    MySQL(String),
    ClickHouse(String),
}

impl TryFrom<(String, String)> for HiqSyncDest {
    type Error = Error;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let typ = value.0.to_lowercase();
        let val = value.1;
        match typ.as_str() {
            "file" => Ok(HiqSyncDest::File(PathBuf::from(val))),
            "mongodb" => Ok(HiqSyncDest::MongoDB(val)),
            "mysql" => Ok(HiqSyncDest::MySQL(val)),
            "clickhouse" => Ok(HiqSyncDest::ClickHouse(val)),
            _ => Err(Error::Custom("invalid HiqSyncDest")),
        }
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HiqSyncDestType {
    File = 1,
    MongoDB,
    MySQL,
    ClickHouse,
}

impl TryFrom<i32> for HiqSyncDestType {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(HiqSyncDestType::File),
            2 => Ok(HiqSyncDestType::MongoDB),
            3 => Ok(HiqSyncDestType::MySQL),
            4 => Ok(HiqSyncDestType::ClickHouse),
            _ => Err(Error::Custom("invalid HiqSyncDestType")),
        }
    }
}



#[derive(Debug, Clone)]
pub enum HiqSyncData {
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
pub enum HiqSyncDataType {
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

impl TryFrom<i32> for HiqSyncDataType {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(HiqSyncDataType::TradeDate),
            // stock
            2 => Ok(HiqSyncDataType::IndexInfo),
            3 => Ok(HiqSyncDataType::IndexBar),
            4 => Ok(HiqSyncDataType::StockInfo),
            5 => Ok(HiqSyncDataType::StockBar),
            6 => Ok(HiqSyncDataType::StockIndex),
            7 => Ok(HiqSyncDataType::StockIndustry),
            8 => Ok(HiqSyncDataType::StockIndustryDetail),
            9 => Ok(HiqSyncDataType::StockIndustryBar),
            10 => Ok(HiqSyncDataType::StockConcept),
            11 => Ok(HiqSyncDataType::StockConceptDetail),
            12 => Ok(HiqSyncDataType::StockConceptBar),
            13 => Ok(HiqSyncDataType::StockYJBB),
            14 => Ok(HiqSyncDataType::StockMargin),

            // fund
            15 => Ok(HiqSyncDataType::FundInfo),
            16 => Ok(HiqSyncDataType::FundNet),
            17 => Ok(HiqSyncDataType::FundBar),

            // bond
            18 => Ok(HiqSyncDataType::BondInfo),
            19 => Ok(HiqSyncDataType::BondBar),
            _ => Err(Error::Custom("invalid HiqSyncDataType")),
        }
    }
}

impl TryFrom<&str> for HiqSyncDataType {
    type Error = Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let v = v.to_lowercase();

        match v.as_str() {
            "trade_date" => Ok(HiqSyncDataType::TradeDate),
            // stock
            "index_info" => Ok(HiqSyncDataType::IndexInfo),
            "index_daily" => Ok(HiqSyncDataType::IndexBar),
            "stock_info" => Ok(HiqSyncDataType::StockInfo),
            "stock_daily" => Ok(HiqSyncDataType::StockBar),
            "stock_index" => Ok(HiqSyncDataType::StockIndex),
            "stock_industry" => Ok(HiqSyncDataType::StockIndustry),
            "stock_industry_detail" => Ok(HiqSyncDataType::StockIndustryDetail),
            "stock_industry_daily" => Ok(HiqSyncDataType::StockIndustryBar),
            "stock_concept" => Ok(HiqSyncDataType::StockConcept),
            "stock_concept_detail" => Ok(HiqSyncDataType::StockConceptDetail),
            "stock_concept_daily" => Ok(HiqSyncDataType::StockConceptBar),
            "stock_yjbb" => Ok(HiqSyncDataType::StockYJBB),
            "stock_margin" => Ok(HiqSyncDataType::StockMargin),

            // fund
            "fund_info" => Ok(HiqSyncDataType::FundInfo),
            "fund_net" => Ok(HiqSyncDataType::FundNet),
            "fund_daily" => Ok(HiqSyncDataType::FundBar),

            // bond
            "bond_info" => Ok(HiqSyncDataType::BondInfo),
            "bond_daily" => Ok(HiqSyncDataType::BondBar),
            _ => Err(Error::Custom("invalid HiqSyncDataType")),
        }
    }
}
