use std::collections::HashSet;

use async_trait::async_trait;
use mongodb::{bson::doc, Client};
use tokio::sync::mpsc;

use crate::{
    store::TAB_STOCK_CONCEPT,
    syncer::{retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use super::service::{insert_many, query};

struct StockConceptAsyncFunc;

#[async_trait]
impl AsyncFunc for StockConceptAsyncFunc {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_stock_concept().await?;

        Ok(Some(SyncData::StockConcept(data)))
    }
}

pub(crate) struct StockConceptSyncer {
    client: Client,
}

impl StockConceptSyncer {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Syncer for StockConceptSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        log::info!("start fetch {}", TAB_STOCK_CONCEPT);
        let func = StockConceptAsyncFunc {};
        let data = retry(func).await?;
        if let Some(data) = data {
            if let SyncData::StockConcept(info) = data {
                let db_data: Vec<rwqfetch::StockConcept> =
                    query(self.client.clone(), TAB_STOCK_CONCEPT, doc! {}, None).await?;

                let set: HashSet<_> = db_data.into_iter().map(|e| e.code).collect();

                let data: Vec<_> = if !set.is_empty() {
                    info.into_iter()
                        .filter(|e| !set.contains(&e.code))
                        .collect()
                } else {
                    info
                };

                if data.len() > 0 {
                    tx.send(SyncData::StockConcept(data)).map_err(|e| {
                        log::error!("send data error {:?}", e);
                        Error::Custom(format!("send data error {:?}", e))
                    })?;
                }
            }
        }
        log::info!("end fetch {}", TAB_STOCK_CONCEPT);
        Ok(())
    }
    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::StockConcept(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_CONCEPT,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_CONCEPT, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_CONCEPT,
                len
            );
        }
        Ok(())
    }
}
