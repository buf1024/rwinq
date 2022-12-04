use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use mongodb::Client;
use tokio::sync::mpsc;

use crate::{
    store::{HiqCache, TAB_STOCK_INFO},
    syncer::{retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

use super::service::insert_many;

struct StockInfoAsyncFunc {
    cache: Arc<RwLock<HiqCache>>,
}

#[async_trait]
impl AsyncFunc for StockInfoAsyncFunc {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = {
            let mut data = Vec::new();
            let cache_info = self.cache.read().unwrap();
            if let Some(info) = cache_info.stock_info() {
                for (_, v) in info.iter() {
                    data.push((*v).clone());
                }
            }
            data
        };
        Ok(Some(HiqSyncData::StockInfo(data)))
    }
}

pub(crate) struct StockInfoSyncer {
    cache: Arc<RwLock<HiqCache>>,
    client: Client,
}

impl StockInfoSyncer {
    pub fn new(client: Client, cache: Arc<RwLock<HiqCache>>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl Syncer for StockInfoSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        let func = StockInfoAsyncFunc {
            cache: self.cache.clone(),
        };
        let data = retry(func).await?;
        if let Some(data) = data {
            tx.send(data).map_err(|e| {
                log::error!("send data error {:?}", e);
                Error::Custom("queue send error")
            })?;
        }
        Ok(())
    }
    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::StockInfo(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INFO,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_INFO, &info, true).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INFO,
                len
            );
        }
        Ok(())
    }
}
