use std::time::Duration;

use async_trait::async_trait;
use chrono::{Local, NaiveDate, Timelike};
use tokio::sync::mpsc;

use crate::{types::HiqSyncData, Result};

#[async_trait]
pub trait AsyncFunc {
    async fn call(&self) -> Result<Option<HiqSyncData>>;
}

pub async fn retry(func: impl AsyncFunc) -> Result<Option<HiqSyncData>> {
    let mut backoff = 1;
    const MAX_BACKOFF: u64 = 64;
    loop {
        let rest = func.call().await;
        if let Err(err) = rest {
            log::error!("fetch remote data error: {:?}, retry later.", &err);
            tokio::time::sleep(Duration::from_millis(backoff * 500)).await;
            if backoff >= MAX_BACKOFF {
                return Err(err);
            }
            backoff *= 2;
            continue;
        }
        return rest;
    }
}

pub fn need_to_start(start: &Option<NaiveDate>) -> bool {
    if let Some(s) = start {
        let now = Local::now().naive_local();
        let hour = now.hour();
        let min = now.minute();
        let n = now.date();
        if s == &n {
            if hour <= 15 {
                if hour == 15 && min > 5 {
                    return true;
                }
                return false;
            }
        }
        if s > &n {
            return false;
        }
    }
    return true;
}

#[async_trait]
pub trait Syncer: Sync + Send {
    async fn fetch(&self, _tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()>;

    // async fn fetch_raw_data(&self) -> Result<Option<HiqSyncData>>;
    async fn save(&self, data: HiqSyncData) -> Result<()>;
}
