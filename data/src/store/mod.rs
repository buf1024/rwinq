use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    sync::Arc,
};

use chrono::{Duration, NaiveDate};
use hiq_fetch::{BondInfo, FundInfo, StockInfo};
use mongodb::bson::Document;

use crate::{
    syncer::Syncer,
    types::HiqSyncDest,
    types::{HiqSyncDataType, HiqSyncDestType},
    Result,
};

use async_trait::async_trait;

mod clickhouse;
pub mod mongo;
mod mysql;
mod sqlite;

pub(crate) use mongo::MongoStore;

use self::mongo::MongoLoader;
// pub(crate) use sqlite::SqliteStore;
// pub(crate) use mysql::MysqlStore;
// pub(crate) use clickhouse::ClickHouseStore;

pub async fn get_store(
    dest: &HiqSyncDest,
    skip_basic: bool,
    split_count: usize,
    funcs: &Option<Vec<HiqSyncDataType>>,
    try_init: bool,
) -> Result<(HiqSyncDestType, Box<dyn Store>)> {
    match dest {
        HiqSyncDest::File(_) => todo!(),
        HiqSyncDest::MongoDB(url) => {
            let mut store: Box<dyn Store> = Box::new(MongoStore::new(
                (*url).clone(),
                skip_basic,
                split_count,
                funcs,
            ));
            if try_init {
                store.init().await?;
            }
            Ok((HiqSyncDestType::MongoDB, store))
        }
        HiqSyncDest::MySQL(_) => todo!(),
        HiqSyncDest::ClickHouse(_) => todo!(),
        // HiqSyncDest::ClickHouse(url) => (HiqSyncDestType::ClickHouse, Box::new(ClickHouseStore::new(url))),
    }
}

pub async fn get_loader(
    dest: &HiqSyncDest,
    try_init: bool,
) -> Result<(HiqSyncDestType, Box<dyn Loader>)> {
    match dest {
        HiqSyncDest::File(_) => todo!(),
        HiqSyncDest::MongoDB(url) => {
            let mut loader = MongoLoader::new((*url).clone());
            if try_init {
                loader.init().await?;
            }
            Ok((HiqSyncDestType::MongoDB, Box::new(loader)))
        }
        HiqSyncDest::MySQL(_) => todo!(),
        HiqSyncDest::ClickHouse(_) => todo!(),
        // HiqSyncDest::ClickHouse(url) => (HiqSyncDestType::ClickHouse, Box::new(ClickHouseStore::new(url))),
    }
}

#[async_trait]
pub trait Store: Sync + Send {
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }
    async fn build_index(&self) -> Result<()> {
        Ok(())
    }
    fn syncer(&self) -> Result<Vec<Arc<Box<dyn Syncer>>>>;
}

pub(crate) struct HiqCache {
    trade_date: Option<HashSet<i32>>,
    index_info: Option<HashMap<String, StockInfo>>,
    stock_info: Option<HashMap<String, StockInfo>>,
    bond_info: Option<HashMap<String, BondInfo>>,
    fund_info: Option<HashMap<String, FundInfo>>,
}

impl HiqCache {
    pub fn new() -> Self {
        Self {
            trade_date: None,
            index_info: None,
            stock_info: None,
            bond_info: None,
            fund_info: None,
        }
    }
    pub fn next_trade_date(&self, date: &NaiveDate) -> NaiveDate {
        let trade_date = if let Some(cache) = &self.trade_date {
            let mut next_date = date.clone();
            loop {
                next_date = next_date.add(Duration::days(1));
                let d: i32 = next_date.format("%Y%m%d").to_string().parse().unwrap();

                if cache.contains(&d) {
                    break next_date;
                }
            }
        } else {
            date.clone()
        };
        trade_date
    }
    pub fn cache_trade_date(&mut self, data: &HashSet<i32>) {
        let mut cache = HashSet::new();
        cache.extend(data.iter());
        self.trade_date = Some(cache);
    }
    pub fn trade_date(&self) -> &Option<HashSet<i32>> {
        &self.trade_date
    }
    pub fn cache_index_info(&mut self, data: &Vec<StockInfo>) {
        let cache: HashMap<String, StockInfo> =
            data.iter().map(|s| (s.code.clone(), s.clone())).collect();
        self.index_info = Some(cache)
    }
    pub fn index_info(&self) -> &Option<HashMap<String, StockInfo>> {
        &self.index_info
    }
    pub fn cache_stock_info(&mut self, data: &Vec<StockInfo>) {
        let cache: HashMap<String, StockInfo> =
            data.iter().map(|s| (s.code.clone(), s.clone())).collect();
        self.stock_info = Some(cache)
    }
    pub fn stock_info(&self) -> &Option<HashMap<String, StockInfo>> {
        &self.stock_info
    }
    pub fn cache_bond_info(&mut self, data: &Vec<BondInfo>) {
        let cache: HashMap<String, BondInfo> =
            data.iter().map(|s| (s.code.clone(), s.clone())).collect();
        self.bond_info = Some(cache)
    }
    pub fn bond_info(&self) -> &Option<HashMap<String, BondInfo>> {
        &self.bond_info
    }
    pub fn cache_fund_info(&mut self, data: &Vec<FundInfo>) {
        let cache: HashMap<String, FundInfo> =
            data.iter().map(|s| (s.code.clone(), s.clone())).collect();
        self.fund_info = Some(cache)
    }
    pub fn fund_info(&self) -> &Option<HashMap<String, FundInfo>> {
        &self.fund_info
    }
}

#[async_trait]
pub trait Loader: Sync + Send {
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }
    async fn load_bond_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::BondInfo>>;
    async fn load_bond_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>>;

    async fn load_fund_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::FundInfo>>;
    async fn load_fund_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>>;
    async fn load_fund_net(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::FundNet>>;

    async fn load_index_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockInfo>>;

    async fn load_index_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>>;

    async fn load_stock_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockInfo>>;

    async fn load_stock_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>>;

    async fn load_stock_index(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockIndex>>;
    async fn load_stock_industry(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockIndustry>>;

    async fn load_stock_industry_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>>;
    async fn load_stock_industry_detail(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockIndustryDetail>>;

    async fn load_stock_concept(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockConcept>>;

    async fn load_stock_concept_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>>;
    async fn load_stock_concept_detail(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockConceptDetail>>;

    async fn load_stock_yjbb(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockYJBB>>;

    async fn load_stock_margin(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::StockMargin>>;
}

pub const DATA_DEF_START_DATE: &'static str = "2010-01-01";

pub const DATABASE: &'static str = "hiq";

pub const TAB_TRADE_DATE: &'static str = "trade_date";

pub const TAB_BOND_INFO: &'static str = "bond_info";
pub const TAB_BOND_DAILY: &'static str = "bond_daily";

pub const TAB_FUND_INFO: &'static str = "fund_info";
pub const TAB_FUND_DAILY: &'static str = "fund_daily";
pub const TAB_FUND_NET: &'static str = "fund_net";

pub const TAB_INDEX_INFO: &'static str = "index_info";
pub const TAB_INDEX_DAILY: &'static str = "index_daily";

pub const TAB_STOCK_INFO: &'static str = "stock_info";
pub const TAB_STOCK_DAILY: &'static str = "stock_daily";
pub const TAB_STOCK_INDEX: &'static str = "stock_index";
pub const TAB_STOCK_INDUSTRY: &'static str = "stock_industry";
pub const TAB_STOCK_INDUSTRY_DAILY: &'static str = "stock_industry_daily";
pub const TAB_STOCK_INDUSTRY_DETAIL: &'static str = "stock_industry_detail";
pub const TAB_STOCK_CONCEPT: &'static str = "stock_concept";
pub const TAB_STOCK_CONCEPT_DAILY: &'static str = "stock_concept_daily";
pub const TAB_STOCK_CONCEPT_DETAIL: &'static str = "stock_concept_detail";
pub const TAB_STOCK_YJBB: &'static str = "stock_yjbb";
pub const TAB_STOCK_MARGIN: &'static str = "stock_margin";
