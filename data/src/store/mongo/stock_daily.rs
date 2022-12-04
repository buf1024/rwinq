use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_fetch::{BarFreq, StockFetch, StockInfo};
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{mongo::service::query_one, HiqCache, DATA_DEF_START_DATE, TAB_STOCK_DAILY},
    syncer::{need_to_start, retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

use super::service::insert_many;

struct StockDailyAsyncFunc<'a> {
    fetch: Arc<dyn StockFetch>,
    code: &'a str,
    name: &'a str,
    freq: Option<BarFreq>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

#[async_trait]
impl<'a> AsyncFunc for StockDailyAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = self
            .fetch
            .fetch_stock_bar(self.code, Some(self.name), self.freq, self.start, self.end)
            .await?;
        let bar = data.bars;
        if bar.is_none() {
            Ok(None)
        } else {
            Ok(Some(HiqSyncData::StockBar(bar.unwrap())))
        }
    }
}

pub(crate) struct StockDailySyncer {
    fetch: Arc<dyn StockFetch>,
    cache: Arc<RwLock<HiqCache>>,
    client: Client,
    codes: Vec<StockInfo>,
    task_n: usize,
}

impl StockDailySyncer {
    pub fn new(
        client: Client,
        fetch: Arc<dyn StockFetch>,
        cache: Arc<RwLock<HiqCache>>,
        codes: Vec<StockInfo>,
        task_n: usize,
    ) -> Self {
        Self {
            client,
            cache,
            fetch,
            codes,
            task_n,
        }
    }
}

#[async_trait]
impl Syncer for StockDailySyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        for info in self.codes.iter() {
            log::info!(
                "prepare sync {}({}) {}, task#{}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_DAILY,
                self.task_n
            );
            let bar: Option<hiq_fetch::Bar> = query_one(
                self.client.clone(),
                TAB_STOCK_DAILY,
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
                    "{}({}) {} is the newest, task#{}",
                    info.name.as_str(),
                    info.code.as_str(),
                    TAB_STOCK_DAILY,
                    self.task_n
                );
                continue;
            }

            log::info!(
                "start fetch {}({}) {}, start={:?}, end=None, task#{}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_DAILY,
                &start,
                self.task_n
            );
            let func = StockDailyAsyncFunc {
                fetch: self.fetch.clone(),
                code: info.code.as_str(),
                name: info.name.as_str(),
                freq: Some(BarFreq::Daily),
                start,
                end: None,
            };
            let data = retry(func).await?;
            if let Some(data) = data {
                tx.send(data).map_err(|e| {
                    log::error!("send data error {:?}", e);
                    Error::Custom("queue send error")
                })?;
            };
            log::info!(
                "end fetch {}({}) {}, start={:?}, end=None, task#{}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_DAILY,
                &start,
                self.task_n
            );
        }

        Ok(())
    }

    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::StockBar(info) = data {
            let bar = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}, task#{}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_DAILY,
                len,
                self.task_n
            );
            insert_many(self.client.clone(), TAB_STOCK_DAILY, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}, task#{}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_DAILY,
                len,
                self.task_n
            );
        }
        Ok(())
    }
}
