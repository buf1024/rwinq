use std::collections::HashSet;

use async_trait::async_trait;
use mongodb::{bson::doc, Client};
use tokio::sync::mpsc;

use crate::{
    store::mongo::service::{insert_many, query},
    syncer::{retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use crate::store::{TAB_STOCK_INDUSTRY, TAB_STOCK_INDUSTRY_DETAIL};

struct StockIndustryDetailAsyncFunc<'a> {
    code: &'a str,
    name: &'a str,
}

#[async_trait]
impl<'a> AsyncFunc for StockIndustryDetailAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_stock_industry_detail(Some(self.code), Some(self.name)).await?;

        Ok(Some(SyncData::StockIndustryDetail(data)))
    }
}

pub(crate) struct StockIndustryDetailSyncer {
    client: Client,
}

impl StockIndustryDetailSyncer {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Syncer for StockIndustryDetailSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        let mut industry: Vec<rwqfetch::StockIndustry> =
            query(self.client.clone(), TAB_STOCK_INDUSTRY, doc! {}, None).await?;
        if industry.is_empty() {
            industry = rwqfetch::fetch_stock_industry().await?;
        }

        for info in industry.iter() {
            log::info!(
                "start sync {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_INDUSTRY_DETAIL
            );
            let func = StockIndustryDetailAsyncFunc {
                code: info.code.as_str(),
                name: info.name.as_str(),
            };
            let code = info.code.as_str();
            let data = retry(func).await?;
            if let Some(data) = data {
                if let SyncData::StockIndustryDetail(info) = data {
                    let db_data: Vec<rwqfetch::StockIndustryDetail> = query(
                        self.client.clone(),
                        TAB_STOCK_INDUSTRY_DETAIL,
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
                        tx.send(SyncData::StockIndustryDetail(data)).map_err(|e| {
                            log::error!("send data error {:?}", e);
                            Error::Custom(format!("send data error {:?}", e))
                        })?;
                    }
                }
            }
            log::info!(
                "end fetch {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_INDUSTRY_DETAIL
            );
        }

        Ok(())
    }

    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::StockIndustryDetail(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INDUSTRY_DETAIL,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_INDUSTRY_DETAIL, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_STOCK_INDUSTRY_DETAIL,
                len
            );
        }
        Ok(())
    }
}
