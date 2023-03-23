use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_fetch::StockFetch;
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{
        mongo::service::{insert_many, query, query_one},
        HiqCache, DATA_DEF_START_DATE, TAB_STOCK_CONCEPT, TAB_STOCK_CONCEPT_DAILY,
    },
    syncer::{need_to_start, retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

struct StockConceptDailyAsyncFunc<'a> {
    fetch: Arc<dyn StockFetch>,
    code: &'a str,
    name: &'a str,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

#[async_trait]
impl<'a> AsyncFunc for StockConceptDailyAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = self
            .fetch
            .fetch_stock_concept_daily(self.code, Some(self.name), self.start, self.end, true)
            .await?;
        let bar = data.bars;
        if bar.is_none() {
            Ok(None)
        } else {
            Ok(Some(HiqSyncData::StockConceptBar(bar.unwrap())))
        }
    }
}

pub(crate) struct StockConceptDailySyncer {
    fetch: Arc<dyn StockFetch>,
    client: Client,
    cache: Arc<RwLock<HiqCache>>,
}

impl StockConceptDailySyncer {
    pub fn new(client: Client, fetch: Arc<dyn StockFetch>, cache: Arc<RwLock<HiqCache>>) -> Self {
        Self {
            client,
            fetch,
            cache,
        }
    }
}

#[async_trait]
impl Syncer for StockConceptDailySyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        let mut concept: Vec<hiq_fetch::StockConcept> =
            query(self.client.clone(), TAB_STOCK_CONCEPT, doc! {}, None).await?;
        if concept.is_empty() {
            concept = self.fetch.fetch_stock_concept().await?;
        }

        for info in concept.iter() {
            log::info!(
                "prepare sync {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_CONCEPT_DAILY
            );
            let bar: Option<hiq_fetch::Bar> = query_one(
                self.client.clone(),
                TAB_STOCK_CONCEPT_DAILY,
                doc! {"code": info.code.as_str()},
                FindOptions::builder()
                    .sort(doc! {"trade_date": -1})
                    .limit(1)
                    .build(),
            )
            .await?;
            let start = bar
                .map(|b| {
                    let cache = self.cache.read().unwrap();
                    Some(cache.next_trade_date(&(b.trade_date.date())))
                })
                .unwrap_or(Some(
                    NaiveDate::parse_from_str(DATA_DEF_START_DATE, "%Y-%m-%d").unwrap(),
                ));
            if !need_to_start(&start) {
                log::info!(
                    "{}({}) {} is the newest",
                    info.name.as_str(),
                    info.code.as_str(),
                    TAB_STOCK_CONCEPT_DAILY,
                );
                continue;
            }

            log::info!(
                "start sync {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_CONCEPT_DAILY,
                &start
            );
            let func = StockConceptDailyAsyncFunc {
                fetch: self.fetch.clone(),
                code: info.code.as_str(),
                name: info.name.as_str(),
                start,
                end: None,
            };
            let data = retry(func).await?;
            if let Some(data) = data {
                tx.send(data).map_err(|e| {
                    log::error!("send data error {:?}", e);
                    Error::Custom(format!("send data error {:?}", e))
                })?;
            };
            log::info!(
                "end fetch {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_CONCEPT_DAILY,
                &start
            );
        }

        Ok(())
    }

    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::StockConceptBar(info) = data {
            let bar = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_CONCEPT_DAILY,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_CONCEPT_DAILY, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_CONCEPT_DAILY,
                len
            );
        }
        Ok(())
    }
}
