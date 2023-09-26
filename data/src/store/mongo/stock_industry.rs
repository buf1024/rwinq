use std::collections::HashSet;

use async_trait::async_trait;
use mongodb::{bson::doc, Client};
use tokio::sync::mpsc;

use crate::{
    store::TAB_STOCK_INDUSTRY,
    syncer::{retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use super::service::{insert_many, query};

struct StockIndustryAsyncFunc;

#[async_trait]
impl AsyncFunc for StockIndustryAsyncFunc {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_stock_industry().await?;

        Ok(Some(SyncData::StockIndustry(data)))
    }
}

pub(crate) struct StockIndustrySyncer {
    client: Client,
}

impl StockIndustrySyncer {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Syncer for StockIndustrySyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        log::info!("start fetch {}", TAB_STOCK_INDUSTRY);
        let func = StockIndustryAsyncFunc {};
        let data = retry(func).await?;
        if let Some(data) = data {
            if let SyncData::StockIndustry(info) = data {
                let db_data: Vec<rwqfetch::StockIndustry> =
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
                    tx.send(SyncData::StockIndustry(data)).map_err(|e| {
                        log::error!("send data error {:?}", e);
                        Error::Custom(format!("send data error {:?}", e))
                    })?;
                }
            }
        }
        log::info!("end fetch {}", TAB_STOCK_INDUSTRY);
        Ok(())
    }
    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::StockIndustry(info) = data {
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
