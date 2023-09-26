use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::NaiveDate;
use mongodb::{bson::doc, options::FindOptions, Client};
use rwqfetch::BarFreq;
use tokio::sync::mpsc;

use crate::{
    store::{mongo::service::query_one, Cache, DATA_DEF_START_DATE, TAB_INDEX_DAILY},
    syncer::{need_to_start, retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use super::service::insert_many;

struct IndexDailyAsyncFunc<'a> {
    code: &'a str,
    name: &'a str,
    freq: Option<BarFreq>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

#[async_trait]
impl<'a> AsyncFunc for IndexDailyAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_index_bar(
            self.code,
            Some(self.name),
            self.freq,
            self.start,
            self.end,
            true,
        )
        .await?;
        let bar = data.bars;
        if bar.is_none() {
            Ok(None)
        } else {
            Ok(Some(SyncData::IndexBar(bar.unwrap())))
        }
    }
}

pub(crate) struct IndexDailySyncer {
    cache: Arc<RwLock<Cache>>,
    client: Client,
}

impl IndexDailySyncer {
    pub fn new(client: Client, cache: Arc<RwLock<Cache>>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl Syncer for IndexDailySyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        let data = {
            let mut data = Vec::new();
            let cache_info = self.cache.read().unwrap();
            if let Some(info) = cache_info.index_info() {
                for (_, v) in info.iter() {
                    data.push((*v).clone());
                }
            }
            data
        };
        for info in data.iter() {
            log::info!(
                "prepare sync {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_INDEX_DAILY
            );
            let bar: Option<rwqfetch::Bar> = query_one(
                self.client.clone(),
                TAB_INDEX_DAILY,
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
                    TAB_INDEX_DAILY,
                );
                continue;
            }

            log::info!(
                "start sync {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_INDEX_DAILY,
                &start
            );
            let func = IndexDailyAsyncFunc {
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
                    Error::Custom(format!("send data error {:?}", e))
                })?;
            };
            log::info!(
                "end fetch {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_INDEX_DAILY,
                &start
            );
        }

        Ok(())
    }

    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::IndexBar(info) = data {
            let bar = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_INDEX_DAILY,
                len
            );
            insert_many(self.client.clone(), TAB_INDEX_DAILY, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_INDEX_DAILY,
                len
            );
        }
        Ok(())
    }
}
