use async_trait::async_trait;
use mongodb::Client;
use tokio::sync::mpsc;

use crate::{
    store::TAB_STOCK_INDEX,
    syncer::{retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use super::service::insert_many;

struct StockIndexAsyncFunc;

#[async_trait]
impl AsyncFunc for StockIndexAsyncFunc {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_stock_index(None).await?;

        if data.is_empty() {
            return Ok(None);
        }

        let data: Vec<_> = data.into_iter().map(|(_, e)| e).collect();

        Ok(Some(SyncData::StockIndex(data)))
    }
}

pub(crate) struct StockIndexSyncer {
    client: Client,
}

impl StockIndexSyncer {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Syncer for StockIndexSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        log::info!("start sync {}", TAB_STOCK_INDEX);
        let func = StockIndexAsyncFunc {};
        let data = retry(func).await?;
        if let Some(data) = data {
            tx.send(data).map_err(|e| {
                log::error!("send data error {:?}", e);
                Error::Custom(format!("send data error {:?}", e))
            })?;
        };
        log::info!("done fetch {}", TAB_STOCK_INDEX);

        Ok(())
    }

    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::StockIndex(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INDEX,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_INDEX, &info, true).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INDEX,
                len
            );
        }
        Ok(())
    }
}
