use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{HiqCache, TAB_TRADE_DATE},
    syncer::{retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

use super::service::{insert_many, query_one};

struct TradeDateAsyncFunc {
    cache: Arc<RwLock<HiqCache>>,
}

#[async_trait]
impl AsyncFunc for TradeDateAsyncFunc {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = {
            let mut data = Vec::new();
            let cache_info = self.cache.read().unwrap();
            if let Some(info) = cache_info.trade_date() {
                for v in info.iter() {
                    data.push(hiq_fetch::TradeDate { trade_date: *v });
                }
            }
            data
        };
        Ok(Some(HiqSyncData::TradeDate(data)))
    }
}

pub(crate) struct TradeDateSyncer {
    cache: Arc<RwLock<HiqCache>>,
    client: Client,
}

impl TradeDateSyncer {
    pub fn new(client: Client, cache: Arc<RwLock<HiqCache>>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl Syncer for TradeDateSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        let func = TradeDateAsyncFunc {
            cache: self.cache.clone(),
        };
        let data = retry(func).await?;
        if let Some(HiqSyncData::TradeDate(trade_date)) = data {
            let latest: Option<hiq_fetch::TradeDate> = query_one(
                self.client.clone(),
                TAB_TRADE_DATE,
                doc! {},
                FindOptions::builder()
                    .sort(doc! {"trade_date": -1})
                    .limit(1)
                    .build(),
            )
            .await?;

            let latest = latest.unwrap_or(hiq_fetch::TradeDate {
                trade_date: 19700101,
            });
            let new_data: Vec<_> = trade_date
                .into_iter()
                .filter(|e| e.trade_date > latest.trade_date)
                .collect();
            if new_data.len() > 0 {
                tx.send(HiqSyncData::TradeDate(new_data)).map_err(|e| {
                    log::error!("send data error {:?}", e);
                    Error::Custom("queue send error")
                })?;
            }
        }

        Ok(())
    }

    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::TradeDate(info) = data {
            let len = info.len();
            log::info!("start save {}, size={}", TAB_TRADE_DATE, len);
            insert_many(self.client.clone(), TAB_TRADE_DATE, &info, false).await?;
            log::info!("done save {}, size={}", TAB_TRADE_DATE, len);
        }
        Ok(())
    }
}
