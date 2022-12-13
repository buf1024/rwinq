use std::{
    collections::{BTreeSet, HashMap},
    ops::Add,
    sync::Arc,
};

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use hiq_fetch::{BondInfo, FundInfo, StockInfo};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

use crate::{
    syncer::Syncer,
    types::HiqSyncDest,
    types::{HiqSyncDataType, HiqSyncDestType},
    Error, Result,
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

/// 获取同步数据store  
/// `dest`: 目标数据源  
/// `skip_basic` 初始化数据是否从远程获取，true在从数据库获取, false则从远程获取    
/// `split_count` 代码切分份数，同一份数据在同一个task里处理  
/// `funcs` 过滤的同步类型，None则全部同步
/// `try_init` 是否初始化
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

/// 获取访问本地数据loader
/// `dest`: 目标数据源  
/// `try_init` 是否初始化
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

/// 同步数据trait接口
#[async_trait]
pub trait Store: Sync + Send {
    /// 初始化
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }
    /// 创建索引
    async fn build_index(&self) -> Result<()> {
        Ok(())
    }
    /// 同步的syncer
    fn syncer(&self) -> Result<Vec<Arc<Box<dyn Syncer>>>>;
}

pub(crate) struct HiqCache {
    trade_date: Option<BTreeSet<i32>>,
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
    pub fn cache_trade_date(&mut self, data: &BTreeSet<i32>) {
        let mut cache = BTreeSet::new();
        cache.extend(data.iter());
        self.trade_date = Some(cache);
    }
    pub fn trade_date(&self) -> &Option<BTreeSet<i32>> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    Bond = 1,
    Fund,
    Stock,
    Index,
    Concept,
    Industry,
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

    async fn load_info(
        &self,
        typ: DataType,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<(String, String)>> {
        let data: Vec<_> = match typ {
            DataType::Bond => self
                .load_bond_info(filter, sort, limit)
                .await?
                .into_iter()
                .map(|e| (e.code, e.name))
                .collect(),
            DataType::Fund => self
                .load_fund_info(filter, sort, limit)
                .await?
                .into_iter()
                .map(|e| (e.code, e.name))
                .collect(),
            DataType::Stock => self
                .load_stock_info(filter, sort, limit)
                .await?
                .into_iter()
                .map(|e| (e.code, e.name))
                .collect(),
            DataType::Index => self
                .load_index_info(filter, sort, limit)
                .await?
                .into_iter()
                .map(|e| (e.code, e.name))
                .collect(),
            DataType::Concept => self
                .load_stock_concept(filter, sort, limit)
                .await?
                .into_iter()
                .map(|e| (e.code, e.name))
                .collect(),
            DataType::Industry => self
                .load_stock_industry(filter, sort, limit)
                .await?
                .into_iter()
                .map(|e| (e.code, e.name))
                .collect(),
        };
        Ok(data)
    }
    async fn load_daily(
        &self,
        typ: DataType,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<hiq_fetch::Bar>> {
        let data: Vec<_> = match typ {
            DataType::Bond => self.load_bond_daily(filter, sort, limit).await?,
            DataType::Fund => self.load_fund_daily(filter, sort, limit).await?,
            DataType::Stock => self.load_stock_daily(filter, sort, limit).await?,
            DataType::Index => self.load_index_daily(filter, sort, limit).await?,
            DataType::Concept => self.load_stock_concept_daily(filter, sort, limit).await?,
            DataType::Industry => self.load_stock_industry_daily(filter, sort, limit).await?,
        };
        Ok(data)
    }
    #[inline]
    fn naive_date_to_datetime_str(&self, naive_date: &NaiveDate) -> Result<String> {
        let dt = NaiveDateTime::new(*naive_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        self.naive_date_time_to_datetime_str(&dt)
    }

    #[inline]
    fn naive_date_time_to_datetime_str(&self, naive_date_time: &NaiveDateTime) -> Result<String> {
        let value = serde_json::to_value(naive_date_time)
            .map_err(|e| Error::Custom(format!("to_value error: {}", e.to_string())))?;
        let s = value
            .as_str()
            .ok_or(Error::Custom("as_str as_str error".to_owned()))?;
        Ok(s.to_owned())
    }
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
