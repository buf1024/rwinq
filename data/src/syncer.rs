use std::time::Duration;

use async_trait::async_trait;
use chrono::{Local, NaiveDate, Timelike};
use tokio::sync::mpsc;

use crate::{types::HiqSyncData, Result};

/// 封装获取数据函数，方便无参数调用，出错时重试等
#[async_trait]
pub trait AsyncFunc {
    async fn call(&self) -> Result<Option<HiqSyncData>>;
}

/// 出错时重试
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

/// 判断是否可以同步
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

/// 同步接口
#[async_trait]
pub trait Syncer: Sync + Send {
    /// 获取远程数据，如果有数据，则塞进队列
    async fn fetch(&self, _tx: mpsc::UnboundedSender<HiqSyncData>) -> Result<()>;

    /// 保存远程数据，独立任务保存
    async fn save(&self, data: HiqSyncData) -> Result<()>;
}
