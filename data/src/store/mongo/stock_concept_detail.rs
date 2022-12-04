use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use hiq_fetch::StockFetch;
use mongodb::{bson::doc, Client};
use tokio::sync::mpsc;

use crate::{
    store::mongo::service::{insert_many, query},
    syncer::{retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};

use crate::store::{TAB_STOCK_CONCEPT, TAB_STOCK_CONCEPT_DETAIL};

struct StockConceptDetailAsyncFunc<'a> {
    fetch: Arc<dyn StockFetch>,
    code: &'a str,
    name: &'a str,
}

#[async_trait]
impl<'a> AsyncFunc for StockConceptDetailAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = self
            .fetch
            .fetch_stock_concept_detail(Some(self.code), Some(self.name))
            .await?;

        Ok(Some(HiqSyncData::StockConceptDetail(data)))
    }
}

pub(crate) struct StockConceptDetailSyncer {
    fetch: Arc<dyn StockFetch>,
    client: Client,
}

impl StockConceptDetailSyncer {
    pub fn new(client: Client, fetch: Arc<dyn StockFetch>) -> Self {
        Self { client, fetch }
    }
}

#[async_trait]
impl Syncer for StockConceptDetailSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
        let mut concept: Vec<hiq_fetch::StockConcept> =
            query(self.client.clone(), TAB_STOCK_CONCEPT, doc! {}, None).await?;
        if concept.is_empty() {
            concept = self.fetch.fetch_stock_concept().await?;
        }

        for info in concept.iter() {
            log::info!(
                "start sync {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_CONCEPT_DETAIL
            );
            let func = StockConceptDetailAsyncFunc {
                fetch: self.fetch.clone(),
                code: info.code.as_str(),
                name: info.name.as_str(),
            };
            let code = info.code.as_str();
            let data = retry(func).await?;
            if let Some(data) = data {
                if let HiqSyncData::StockConceptDetail(info) = data {
                    let db_data: Vec<hiq_fetch::StockConceptDetail> = query(
                        self.client.clone(),
                        TAB_STOCK_CONCEPT_DETAIL,
                        doc! {"code": code},
                        None,
                    )
                    .await?;

                    let set: HashSet<_> = db_data.into_iter().map(|e| e.stock_code).collect();

                    let data: Vec<_> = if !set.is_empty() {
                        info.into_iter()
                            .filter(|e| !set.contains(&e.stock_code))
                            .collect()
                    } else {
                        info
                    };

                    if data.len() > 0 {
                        tx.send(HiqSyncData::StockConceptDetail(data))
                            .map_err(|e| {
                                log::error!("send data error {:?}", e);
                                Error::Custom("queue send error")
                            })?;
                    }
                }
            }
            log::info!(
                "end fetch {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_CONCEPT_DETAIL
            );
        }

        Ok(())
    }

    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::StockConceptDetail(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_CONCEPT_DETAIL,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_CONCEPT_DETAIL, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_CONCEPT_DETAIL,
                len
            );
        }
        Ok(())
    }
}
