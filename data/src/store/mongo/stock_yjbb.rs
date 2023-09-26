use std::collections::HashSet;

use async_trait::async_trait;
use chrono::{Datelike, Local};
use mongodb::{bson::doc, options::FindOptions, Client};
use tokio::sync::mpsc;

use crate::{
    store::{
        mongo::service::{insert_many, query, query_one},
        TAB_STOCK_YJBB,
    },
    syncer::{retry, AsyncFunc, Syncer},
    types::SyncData,
    Error, Result,
};

struct StockYJBBAsyncFunc {
    year: u16,
    season: u16,
}

#[async_trait]
impl AsyncFunc for StockYJBBAsyncFunc {
    async fn call(&self) -> Result<Option<SyncData>> {
        let data = rwqfetch::fetch_stock_yjbb(self.year, self.season).await?;
        if data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(SyncData::StockYJBB(data)))
        }
    }
}

pub(crate) struct StockYJBBSyncer {
    client: Client,
}

impl StockYJBBSyncer {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Syncer for StockYJBBSyncer {
    async fn fetch(&self, tx: mpsc::UnboundedSender<SyncData>) -> Result<()> {
        let yjbb: Option<rwqfetch::StockYJBB> = query_one(
            self.client.clone(),
            TAB_STOCK_YJBB,
            doc! {},
            FindOptions::builder()
                .sort(doc! {"season_date": -1})
                .limit(1)
                .build(),
        )
        .await?;

        let mut s_vec = Vec::new();
        let n_year = Local::now().naive_local().year() as u16;
        if let Some(yjbb) = yjbb {
            let (year, season) = (yjbb.year, yjbb.season);
            if year == n_year {
                for s in 0..4 - season + 1 {
                    s_vec.push((year, s + 1))
                }
            } else {
                for y in 0..n_year - year + 1 {
                    for s in 0..4 {
                        if y == 0 && s + 1 < season {
                            continue;
                        }
                        s_vec.push((year + y, s + 1));
                    }
                }
            }
        } else {
            for y in 0..n_year - 1991 + 1 {
                for s in 0..4 {
                    s_vec.push((y + 1991, s as u16 + 1))
                }
            }
        }
        for (year, season) in s_vec.into_iter() {
            log::info!(
                "start sync {} year={}, season={}",
                TAB_STOCK_YJBB,
                year,
                season
            );

            let func = StockYJBBAsyncFunc { year, season };
            let data = retry(func).await?;
            if let Some(data) = data {
                if let SyncData::StockYJBB(info) = data {
                    let db_data: Vec<rwqfetch::StockYJBB> = query(
                        self.client.clone(),
                        TAB_STOCK_YJBB,
                        doc! {"year": year as i32, "season": season as i32},
                        None,
                    )
                    .await?;

                    let set: HashSet<_> = db_data.into_iter().map(|e| e.code).collect();

                    let data: Vec<_> = if !set.is_empty() {
                        info.into_iter()
                            .filter(|e| !set.contains(&e.code))
                            .collect()
                    } else {
                        info
                    };

                    if data.len() > 0 {
                        tx.send(SyncData::StockYJBB(data)).map_err(|e| {
                            log::error!("send data error {:?}", e);
                            Error::Custom(format!("send data error {:?}", e))
                        })?;
                    }
                }
            }
            log::info!(
                "end fetch {} year={}, season={}",
                TAB_STOCK_YJBB,
                year,
                season
            );
        }

        Ok(())
    }

    async fn save(&self, data: SyncData) -> Result<()> {
        if let SyncData::StockYJBB(info) = data {
            let bar = info.get(0).unwrap();
            let len = info.len();
            log::info!(
                "start save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_YJBB,
                len
            );
            insert_many(self.client.clone(), TAB_STOCK_YJBB, &info, false).await?;
            log::info!(
                "done save {}({}) {}, size={}",
                bar.name.as_str(),
                bar.code.as_str(),
                TAB_STOCK_YJBB,
                len
            );
        }
        Ok(())
    }
}
