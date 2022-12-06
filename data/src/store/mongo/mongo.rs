use std::{
    collections::BTreeSet,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use hiq_fetch::{BondFetch, FundFetch, StockFetch};
use mongodb::{bson::doc, options::ClientOptions, Client};

use crate::{
    store::{
        mongo::service::query, HiqCache, Store, TAB_BOND_INFO, TAB_FUND_INFO, TAB_INDEX_INFO,
        TAB_STOCK_INFO, TAB_TRADE_DATE,
    },
    syncer::Syncer,
    types::HiqSyncDataType,
    Error, Result,
};

use super::{
    bond_daily::BondDailySyncer, bond_info::BondInfoSyncer, fund_daily::FundDailySyncer,
    fund_info::FundInfoSyncer, fund_net::FundNetSyncer, index_daily::IndexDailySyncer,
    index_info::IndexInfoSyncer, mongo_index::build_index, stock_concept::StockConceptSyncer,
    stock_concept_daily::StockConceptDailySyncer, stock_concept_detail::StockConceptDetailSyncer,
    stock_daily::StockDailySyncer, stock_index::StockIndexSyncer,
    stock_industry::StockIndustrySyncer, stock_industry_daily::StockIndustryDailySyncer,
    stock_industry_detail::StockIndustryDetailSyncer, stock_info::StockInfoSyncer,
    stock_margin::StockMarginSyncer, stock_yjbb::StockYJBBSyncer, trade_date::TradeDateSyncer,
};

pub(crate) struct MongoStore {
    bond_fetch: Arc<dyn BondFetch>,
    fund_fetch: Arc<dyn FundFetch>,
    stock_fetch: Arc<dyn StockFetch>,
    syncer_vec: Vec<Arc<Box<dyn Syncer>>>,
    cache: Arc<RwLock<HiqCache>>,

    url: String,
    skip_basic: bool,
    split_count: usize,
    funcs: Option<Vec<HiqSyncDataType>>,
}

impl MongoStore {
    pub fn new(
        url: String,
        skip_basic: bool,
        split_count: usize,
        funcs: &Option<Vec<HiqSyncDataType>>,
    ) -> Self {
        let bond_fetch = Arc::new(hiq_fetch::bond_fetch());
        let fund_fetch = Arc::new(hiq_fetch::fund_fetch());
        let stock_fetch = Arc::new(hiq_fetch::stock_fetch());

        let syncer_vec = Vec::new();

        let cache = Arc::new(RwLock::new(HiqCache::new()));

        let mut t_funcs = None;
        if let Some(funcs) = funcs {
            let v: Vec<_> = funcs.iter().map(|e| (*e).clone()).collect();
            t_funcs = Some(v);
        }
        Self {
            bond_fetch,
            fund_fetch,
            stock_fetch,
            syncer_vec,
            cache,
            url,
            skip_basic,
            split_count,
            funcs: t_funcs,
        }
    }
    async fn prepare_cache(&mut self, client: Client) -> Result<()> {
        let (bond_info, index_info, stock_info, fund_info, trade_date) = if !self.skip_basic {
            log::info!("prepare cache data from remote");

            log::info!("prepare cache bond_info");
            let bond_info = self.bond_fetch.fetch_bond_info().await?;

            log::info!("prepare cache index_info");
            let index_info = self.stock_fetch.fetch_index_info().await?;

            log::info!("prepare cache stock_info");
            let stock_info = self.stock_fetch.fetch_stock_info().await?;

            log::info!("prepare cache fund_info");
            let fund_info = self.fund_fetch.fetch_fund_info().await?;

            log::info!("prepare cache trade_date");
            let trade_date = hiq_fetch::fetch_trade_date().await?;

            (bond_info, index_info, stock_info, fund_info, trade_date)
        } else {
            log::info!("prepare cache data from database");

            log::info!("prepare cache bond_info");
            let bond_info = query(client.clone(), TAB_BOND_INFO, doc! {}, None).await?;

            log::info!("prepare cache index_info");
            let index_info = query(client.clone(), TAB_INDEX_INFO, doc! {}, None).await?;

            log::info!("prepare cache stock_info");
            let stock_info = query(client.clone(), TAB_STOCK_INFO, doc! {}, None).await?;

            log::info!("prepare cache fund_info");
            let fund_info = query(client.clone(), TAB_FUND_INFO, doc! {}, None).await?;

            log::info!("prepare cache trade_date");
            let trade_date_v: Vec<hiq_fetch::TradeDate> =
                query(client.clone(), TAB_TRADE_DATE, doc! {}, None).await?;

            let trade_date: BTreeSet<_> = trade_date_v.iter().map(|t| t.trade_date).collect();
            (bond_info, index_info, stock_info, fund_info, trade_date)
        };

        {
            let mut cache = self.cache.write().map_err(|e| {
                log::error!("get cache write log error: {}", e.to_string());
                Error::Custom("get cache write log error")
            })?;
            cache.cache_bond_info(&bond_info);
            cache.cache_index_info(&index_info);
            cache.cache_stock_info(&stock_info);
            cache.cache_fund_info(&fund_info);
            cache.cache_trade_date(&trade_date);
        }
        if bond_info.is_empty()
            || index_info.is_empty()
            || stock_info.is_empty()
            || fund_info.is_empty()
            || trade_date.is_empty()
        {
            Err(Error::Custom("cache info is empty, try not skip basic"))
        } else {
            Ok(())
        }
    }
    fn contains(&self, typ: &HiqSyncDataType) -> bool {
        if self.funcs.is_none() {
            return true;
        }
        let funcs = self.funcs.as_ref().unwrap();
        funcs.contains(typ)
    }
    fn add_syncer(&mut self, typ: &HiqSyncDataType, syncer: Arc<Box<dyn Syncer>>) {
        if self.contains(typ) {
            // log::info!("add syncer: {:?}", typ);
            self.syncer_vec.push(syncer);
        }
    }
    fn prepare_heavy_syncer(&mut self, client: Client, split_count: usize) {
        let stock_codes = {
            let mut data = Vec::new();
            let cache_info = self.cache.read().unwrap();
            if let Some(info) = cache_info.stock_info() {
                for (_, v) in info.iter() {
                    data.push((*v).clone());
                }
            }
            data
        };
        let mut task_n = 0;
        let len = stock_codes.len() / split_count;
        let len_n = len * split_count;
        let mut sub_codes = Vec::new();
        let mut margin_sub_codes = Vec::new();
        for (i, code) in stock_codes.into_iter().enumerate() {
            if code.is_margin {
                margin_sub_codes.push(code.clone());
            }
            sub_codes.push(code);
            if i + 1 >= len_n {
                continue;
            }
            if sub_codes.len() >= len {
                task_n += 1;
                self.add_syncer(
                    &HiqSyncDataType::StockBar,
                    Arc::new(Box::new(StockDailySyncer::new(
                        client.clone(),
                        self.stock_fetch.clone(),
                        self.cache.clone(),
                        sub_codes,
                        task_n,
                    ))),
                );

                self.add_syncer(
                    &HiqSyncDataType::StockMargin,
                    Arc::new(Box::new(StockMarginSyncer::new(
                        client.clone(),
                        self.stock_fetch.clone(),
                        self.cache.clone(),
                        margin_sub_codes,
                        task_n,
                    ))),
                );

                sub_codes = Vec::new();
                margin_sub_codes = Vec::new();
            }
        }
        if sub_codes.len() >= len {
            task_n += 1;
            self.add_syncer(
                &HiqSyncDataType::StockBar,
                Arc::new(Box::new(StockDailySyncer::new(
                    client.clone(),
                    self.stock_fetch.clone(),
                    self.cache.clone(),
                    sub_codes.clone(),
                    task_n,
                ))),
            );
        }
        if margin_sub_codes.len() >= len {
            task_n += 1;
            self.add_syncer(
                &HiqSyncDataType::StockMargin,
                Arc::new(Box::new(StockMarginSyncer::new(
                    client.clone(),
                    self.stock_fetch.clone(),
                    self.cache.clone(),
                    margin_sub_codes,
                    task_n,
                ))),
            );
        }
    }
    fn prepare_syncer(&mut self, client: Client, split_count: usize) {
        if !self.skip_basic {
            // bond
            self.add_syncer(
                &HiqSyncDataType::BondInfo,
                Arc::new(Box::new(BondInfoSyncer::new(
                    client.clone(),
                    self.cache.clone(),
                ))),
            );

            // stock
            self.add_syncer(
                &HiqSyncDataType::IndexInfo,
                Arc::new(Box::new(IndexInfoSyncer::new(
                    client.clone(),
                    self.cache.clone(),
                ))),
            );
            self.add_syncer(
                &HiqSyncDataType::StockInfo,
                Arc::new(Box::new(StockInfoSyncer::new(
                    client.clone(),
                    self.cache.clone(),
                ))),
            );

            // fund
            self.add_syncer(
                &HiqSyncDataType::FundInfo,
                Arc::new(Box::new(FundInfoSyncer::new(
                    client.clone(),
                    self.cache.clone(),
                ))),
            );

            // trade_date
            self.add_syncer(
                &HiqSyncDataType::TradeDate,
                Arc::new(Box::new(TradeDateSyncer::new(
                    client.clone(),
                    self.cache.clone(),
                ))),
            );
        }

        // bond
        self.add_syncer(
            &HiqSyncDataType::BondBar,
            Arc::new(Box::new(BondDailySyncer::new(
                client.clone(),
                self.bond_fetch.clone(),
                self.cache.clone(),
            ))),
        );

        // fund
        self.add_syncer(
            &HiqSyncDataType::FundBar,
            Arc::new(Box::new(FundDailySyncer::new(
                client.clone(),
                self.fund_fetch.clone(),
                self.cache.clone(),
            ))),
        );

        self.add_syncer(
            &HiqSyncDataType::FundNet,
            Arc::new(Box::new(FundNetSyncer::new(
                client.clone(),
                self.fund_fetch.clone(),
                self.cache.clone(),
            ))),
        );

        // stock
        self.add_syncer(
            &HiqSyncDataType::IndexBar,
            Arc::new(Box::new(IndexDailySyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
                self.cache.clone(),
            ))),
        );

        self.add_syncer(
            &HiqSyncDataType::StockIndex,
            Arc::new(Box::new(StockIndexSyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
            ))),
        );

        self.add_syncer(
            &HiqSyncDataType::StockIndustry,
            Arc::new(Box::new(StockIndustrySyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
            ))),
        );
        self.add_syncer(
            &HiqSyncDataType::StockIndustryBar,
            Arc::new(Box::new(StockIndustryDailySyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
                self.cache.clone(),
            ))),
        );
        self.add_syncer(
            &HiqSyncDataType::StockIndustryDetail,
            Arc::new(Box::new(StockIndustryDetailSyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
            ))),
        );

        self.add_syncer(
            &HiqSyncDataType::StockConcept,
            Arc::new(Box::new(StockConceptSyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
            ))),
        );
        self.add_syncer(
            &HiqSyncDataType::StockConceptBar,
            Arc::new(Box::new(StockConceptDailySyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
                self.cache.clone(),
            ))),
        );
        self.add_syncer(
            &HiqSyncDataType::StockConceptDetail,
            Arc::new(Box::new(StockConceptDetailSyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
            ))),
        );
        self.add_syncer(
            &HiqSyncDataType::StockYJBB,
            Arc::new(Box::new(StockYJBBSyncer::new(
                client.clone(),
                self.stock_fetch.clone(),
            ))),
        );
        self.prepare_heavy_syncer(client, split_count);
    }

    async fn build_client(&self) -> Result<Client> {
        let mut client_options = ClientOptions::parse(&self.url[..]).await.map_err(|e| {
            log::error!("parse connect url error: {}", e.to_string());
            Error::Custom("parse connect url error")
        })?;

        client_options.app_name = Some("HIQ App".to_string());

        let client = Client::with_options(client_options).map_err(|e| {
            log::error!("with_options error: {}", e.to_string());
            Error::Custom("with client option error")
        })?;
        Ok(client)
    }
}

#[async_trait]
impl Store for MongoStore {
    async fn init(&mut self) -> Result<()> {
        let client = self.build_client().await?;

        self.prepare_cache(client.clone()).await?;
        self.prepare_syncer(client.clone(), self.split_count);

        Ok(())
    }
    async fn build_index(&self) -> Result<()> {
        let client = self.build_client().await?;
        build_index(client).await
    }

    fn syncer(&self) -> Result<Vec<Arc<Box<dyn Syncer>>>> {
        Ok(self.syncer_vec.iter().map(|e| e.clone()).collect())
    }
}

#[cfg(test)]
mod tests {

    use futures::stream::TryStreamExt;
    use mongodb::bson::doc;
    use mongodb::options::{ClientOptions, FindOptions};
    use mongodb::Client;

    async fn test_mongo() -> Result<(), Box<dyn std::error::Error>> {
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        client_options.app_name = Some("My HIQ App".to_string());

        let client = Client::with_options(client_options)?;

        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        let mut info = vec![];
        for i in 0..10 {
            info.push(hiq_fetch::BondInfo {
                code: format!("code-{}", i),
                name: "name1".to_string(),
                stock_code: "股票1code".to_string(),
                stock_name: "股票1name".to_string(),
                listing_date: Default::default(),
                is_delist: 0,
            })
        }

        let db = client.database("hiq");
        let coll = db.collection::<hiq_fetch::BondInfo>("bond_info");
        coll.insert_many(info, None).await?;

        let opt = FindOptions::builder()
            .sort(doc! {"code": -1})
            // .projection(doc! {"code": 1, "stock_code": 1})
            .build();

        let filter = doc! {"name": {"$regex": ".*1"}};
        let mut res = coll.find(filter, opt).await?;

        while let Some(r) = res.try_next().await? {
            print!("result: {:?}", r);
        }

        Ok(())
    }

    #[test]
    fn test_mongo_async() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let res = test_mongo().await;
                if res.is_err() {
                    println!("Error: {:?}", res);
                }
            })
    }
}
