use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use hiq_fetch::StockFetch;
use mongodb::{bson::doc, Client};
use tokio::sync::mpsc;

use crate::{
    store::TAB_STOCK_INDUSTRY,
    syncer::{retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

use super::service::{insert_many, query};

struct StockIndustryAsyncFunc {
    fetch: Arc<dyn StockFetch>,
}

#[async_trait]
impl AsyncFunc for StockIndustryAsyncFunc {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = self.fetch.fetch_stock_industry().await?;

        Ok(Some(HiqSyncData::StockIndustry(data)))
    }
}

pub(crate) struct StockIndustrySyncer {
    fetch: Arc<dyn StockFetch>,
    client: Client,
}

impl StockIndustrySyncer {
    pub fn new(client: Client, fetch: Arc<dyn StockFetch>) -> Self {
        Self { client, fetch }
    }
}

#[async_trait]
impl Syncer for StockIndustrySyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        log::info!("start fetch {}", TAB_STOCK_INDUSTRY);
        let func = StockIndustryAsyncFunc {
            fetch: self.fetch.clone(),
        };
        let data = retry(func).await?;
        if let Some(data) = data {
            if let HiqSyncData::StockIndustry(info) = data {
                let db_data: Vec<hiq_fetch::StockIndustry> =
                    query(self.client.clone(), TAB_STOCK_INDUSTRY, doc! {}, None).await?;

                let set: HashSet<_> = db_data.into_iter().map(|e| e.code).collect();

                let data: Vec<_> = if !set.is_empty() {
                    info.into_iter()
                        .filter(|e| !set.contains(&e.code))
                        .collect()
                } else {
                    info
                };

                if data.len() > 0 {
                    tx.send(HiqSyncData::StockIndustry(data)).map_err(|e| {
                        log::error!("send data error {:?}", e);
                        Error::Custom(format!("send data error {:?}", e))
                    })?;
                }
            }
        }
        log::info!("end fetch {}", TAB_STOCK_INDUSTRY);
        Ok(())
    }
    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::StockIndustry(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INDUSTRY,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_INDUSTRY, &info, true).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INDUSTRY,
                len
            );
        }
        Ok(())
    }
}
