use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use mongodb::Client;
use tokio::sync::mpsc;

use crate::{
    store::{Cache, TAB_INDEX_INFO},
    syncer::{retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use super::service::insert_many;

struct IndexInfoAsyncFunc {
    cache: Arc<RwLock<Cache>>,
}

#[async_trait]
impl AsyncFunc for IndexInfoAsyncFunc {
    async fn call(&self) -> Result<Option<SyncData>> {
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
        Ok(Some(SyncData::IndexInfo(data)))
    }
}

pub(crate) struct IndexInfoSyncer {
    cache: Arc<RwLock<Cache>>,
    client: Client,
}

impl IndexInfoSyncer {
    pub fn new(client: Client, cache: Arc<RwLock<Cache>>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl Syncer for IndexInfoSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        let func = IndexInfoAsyncFunc {
            cache: self.cache.clone(),
        };
        let data = retry(func).await?;
        if let Some(data) = data {
            tx.send(data).map_err(|e| {
                log::error!("send data error {:?}", e);
                Error::Custom(format!("send data error {:?}", e))
            })?;
        }
        Ok(())
    }
    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::IndexInfo(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_INDEX_INFO,
                len
            );
            insert_many(self.client.clone(), TAB_INDEX_INFO, &info, true).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_INDEX_INFO,
                len
            );
        }
        Ok(())
    }
}
