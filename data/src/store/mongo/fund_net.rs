use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_fetch::FundFetch;
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{
        mongo::service::{insert_many, query_one},
        HiqCache, DATA_DEF_START_DATE, TAB_FUND_NET,
    },
    syncer::{need_to_start, retry, AsyncFunc, Syncer},
    types::HiqSyncData,
    Error, Result,
};


struct FundNetAsyncFunc<'a> {
    fetch: Arc<dyn FundFetch>,
    code: &'a str,
    name: &'a str,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

#[async_trait]
impl<'a> AsyncFunc for FundNetAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<HiqSyncData>> {
        let data = self
            .fetch
            .fetch_fund_net(self.code, Some(self.name), self.start, self.end)
            .await?;
        if data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(HiqSyncData::FundNet(data)))
        }
    }
}

pub(crate) struct FundNetSyncer {
    fetch: Arc<dyn FundFetch>,
    cache: Arc<RwLock<HiqCache>>,
    client: Client,
}

impl FundNetSyncer {
    pub fn new(client: Client, fetch: Arc<dyn FundFetch>, cache: Arc<RwLock<HiqCache>>) -> Self {
        Self {
            client,
            fetch,
            cache,
        }
    }
}

#[async_trait]
impl Syncer for FundNetSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()> {
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
        for info in data.iter() {
            let bar: Option<hiq_fetch::FundNet> = query_one(
                self.client.clone(),
                TAB_FUND_NET,
                doc! {"code": info.code.as_str()},
                FindOptions::builder()
                    .sort(doc! {"trade_date": -1})
                    .limit(1)
                    .build(),
            )
            .await?;
            let start = bar
                .map(|b| {
                    let cache = self.cache.read().unwrap();
                    Some(cache.next_trade_date(&(b.trade_date)))
                })
                .unwrap_or(Some(
                    NaiveDate::parse_from_str(DATA_DEF_START_DATE, "%Y-%m-%d").unwrap(),
                ));
            if !need_to_start(&start) {
                log::info!(
                    "{}({}) {} is the newest",
                    info.name.as_str(),
                    info.code.as_str(),
                    TAB_FUND_NET,
                );
                continue;
            }

            log::info!(
                "start sync {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_FUND_NET,
                &start
            );
            let func = FundNetAsyncFunc {
                fetch: self.fetch.clone(),
                code: info.code.as_str(),
                name: info.name.as_str(),
                start,
                end: None,
            };
            let data = retry(func).await?;
            if let Some(data) = data {
                tx.send(data).map_err(|e| {
                    log::error!("send data error {:?}", e);
                    Error::Custom("queue send error")
                })?;
            };
            log::info!(
                "end fetch {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_FUND_NET,
                &start
            );
        }

        Ok(())
    }

    async fn save(&self, data: HiqSyncData) -> Result<()> {
        if let HiqSyncData::FundNet(info) = data {
            let elm = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_FUND_NET,
                len
            );
            insert_many(self.client.clone(), TAB_FUND_NET, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                elm.name.as_str(),
                elm.code.as_str(),
                TAB_FUND_NET,
                len
            );
        }
        Ok(())
    }
}
