use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use mongodb::Client;
use tokio::sync::mpsc;

use crate::{
    store::{mongo::service::insert_many, HiqCache, TAB_FUND_INFO},
    syncer::{retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};


struct FundInfoAsyncFunc {
    cache: Arc<RwLock<HiqCache>>,
}

#[async_trait]
impl AsyncFunc for FundInfoAsyncFunc {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = {
            let mut data = Vec::new();
            let cache_info = self.cache.read().unwrap();
            if let Some(info) = cache_info.fund_info() {
                for (_, v) in info.iter() {
                    data.push((*v).clone());
                }
            }
            data
        };
        Ok(Some(HiqSyncData::FundInfo(data)))
    }
}

pub(crate) struct FundInfoSyncer {
    cache: Arc<RwLock<HiqCache>>,
    client: Client,
}

impl FundInfoSyncer {
    pub fn new(client: Client, cache: Arc<RwLock<HiqCache>>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl Syncer for FundInfoSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        let func = FundInfoAsyncFunc {
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
    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::FundInfo(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_FUND_INFO,
                len
            );
            insert_many(self.client.clone(), TAB_FUND_INFO, &info, true).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_FUND_INFO,
                len
            );
        }
        Ok(())
    }
}
