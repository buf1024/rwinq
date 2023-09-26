use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::NaiveDate;
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{
        mongo::service::{insert_many, query, query_one},
        Cache, DATA_DEF_START_DATE,
    },
    syncer::{need_to_start, retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

use crate::store::{TAB_STOCK_INDUSTRY, TAB_STOCK_INDUSTRY_DAILY};

struct StockIndustryDailyAsyncFunc<'a> {
    code: &'a str,
    name: &'a str,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

#[async_trait]
impl<'a> AsyncFunc for StockIndustryDailyAsyncFunc<'a> {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_stock_industry_daily(
            self.code,
            Some(self.name),
            self.start,
            self.end,
            true,
        )
        .await?;
        let bar = data.bars;
        if bar.is_none() {
            Ok(None)
        } else {
            Ok(Some(SyncData::StockIndustryBar(bar.unwrap())))
        }
    }
}

pub(crate) struct StockIndustryDailySyncer {
    client: Client,
    cache: Arc<RwLock<Cache>>,
}

impl StockIndustryDailySyncer {
    pub fn new(client: Client, cache: Arc<RwLock<Cache>>) -> Self {
        Self { client, cache }
    }
}

#[async_trait]
impl Syncer for StockIndustryDailySyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        let mut industry: Vec<rwqfetch::StockIndustry> =
            query(self.client.clone(), TAB_STOCK_INDUSTRY, doc! {}, None).await?;
        if industry.is_empty() {
            industry = rwqfetch::fetch_stock_industry().await?;
        }

        for info in industry.iter() {
            log::info!(
                "prepare sync {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_INDUSTRY_DAILY
            );
            log::info!(
                "prepare sync {}({}) {}",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_INDUSTRY_DAILY
            );
            let bar: Option<rwqfetch::Bar> = query_one(
                self.client.clone(),
                TAB_STOCK_INDUSTRY_DAILY,
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
                    Some(cache.next_trade_date(&(b.trade_date.date())))
                })
                .unwrap_or(Some(
                    NaiveDate::parse_from_str(DATA_DEF_START_DATE, "%Y-%m-%d").unwrap(),
                ));

            if !need_to_start(&start) {
                log::info!(
                    "{}({}) {} is the newest",
                    info.name.as_str(),
                    info.code.as_str(),
                    TAB_STOCK_INDUSTRY_DAILY,
                );
                continue;
            }

            log::info!(
                "start sync {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_INDUSTRY_DAILY,
                &start
            );
            let func = StockIndustryDailyAsyncFunc {
                code: info.code.as_str(),
                name: info.name.as_str(),
                start,
                end: None,
            };
            let data = retry(func).await?;
            if let Some(data) = data {
                tx.send(data).map_err(|e| {
                    log::error!("send data error {:?}", e);
                    Error::Custom(format!("send data error {:?}", e))
                })?;
            };
            log::info!(
                "end fetch {}({}) {}, start={:?}, end=None",
                info.name.as_str(),
                info.code.as_str(),
                TAB_STOCK_INDUSTRY_DAILY,
                &start
            );
        }

        Ok(())
    }

    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::StockIndustryBar(info) = data {
            let bar = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_INDUSTRY_DAILY,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_INDUSTRY_DAILY, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_INDUSTRY_DAILY,
                len
            );
        }
        Ok(())
    }
}
