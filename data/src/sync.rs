//! 数据同步，支持同时同步到多个数据源
use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;

use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{broadcast, mpsc};

use crate::store::get_store;
use crate::types::SyncDataType;
use crate::{
    store::Store,
    syncer::Syncer,
    types::{SyncData, SyncDest, SyncDestType},
    Error, Result,
};

/// 数据同步
pub struct Sync {
    dest: Vec<SyncDest>,
    shutdown: broadcast::Receiver<()>,
    funcs: Option<Vec<SyncDataType>>,
    store: Option<HashMap<SyncDestType, Arc<Box<dyn Store>>>>,
    is_init: bool,
}

impl Sync {
    /// 构造对象  
    /// `dest` 数据源，需保证数据源正确，否则后续`init`会报错  
    /// `shutdown·` 停止信号  
    /// `funcs` 同步的条目类型，如果为`None`，则全部同步
    pub fn new(
        dest: Vec<SyncDest>,
        shutdown: broadcast::Receiver<()>,
        funcs: Option<Vec<SyncDataType>>,
    ) -> Self {
        Self {
            dest,
            shutdown,
            funcs,
            store: None,
            is_init: false,
        }
    }
    /// 初始化
    /// `skip_basic` 初始化数据是否从远程获取，true在从数据库获取, false则从远程获取    
    /// `split_count` 代码切分份数，同一份数据在同一个task里处理  
    pub async fn init(&mut self, skip_basic: bool, split_count: usize) -> Result<()> {
        if !self.is_init {
            let mut store = HashMap::new();
            for (i, dest) in self.dest.iter().enumerate() {
                let (t, s) = get_store(dest, skip_basic, split_count, &self.funcs, true).await?;
                store.insert(t.clone(), Arc::new(s));
                log::debug!("store#{}{:?}-{:?} inited ", i, dest, &t);
            }
            self.store = Some(store);
            self.is_init = true;
            log::info!("store(s) inited");
        }
        Ok(())
    }
    /// 初始化
    /// `skip_basic` 初始化数据是否从远程获取，true在从数据库获取, false则从远程获取    
    /// `split_count` 代码切分份数，同一份数据在同一个task里处理  
    /// `task_count` 远程获取数据启动的task数量，代表并发获取， task_count不宜过大，可能被封
    pub async fn sync(
        &mut self,
        skip_basic: bool,
        task_count: usize,
        split_count: usize,
    ) -> Result<()> {
        self.init(skip_basic, split_count).await?;
        if let Some(ref store) = self.store {
            let mut fut = Vec::new();

            let (tx, _) = broadcast::channel(1);
            for (typ, store) in store.iter() {
                let store = (*store).clone();
                let rx = tx.subscribe();
                let h = tokio::spawn(sync_task((*typ).clone(), store, rx, task_count));
                fut.push(h);
            }

            tokio::select! {
                _ = join_all(fut) => {},
                _ = self.shutdown.recv() => {
                    log::info!("stop sync received");
                    tx.send(()).map_err(|e| {
                        log::error!("send data error {:?}", e);
                        Error::Custom(format!("send data error {:?}", e))
                    })?;
                }
            }
        }
        log::info!("done sync");
        Ok(())
    }
}

async fn sync_task(
    store_typ: SyncDestType,
    store: Arc<Box<dyn Store>>,
    mut shutdown_rx: broadcast::Receiver<()>,
    task_count: usize,
) -> Result<()> {
    log::info!("start sync {:?}", &store_typ);

    let (shutdown_tx, _) = broadcast::channel(1);

    let mut fut = Vec::new();
    let mut tx_map = HashMap::new();
    let syncer = store.syncer()?;
    for (i, s) in syncer.iter().enumerate() {
        let (tx, rx) = mpsc::unbounded_channel();
        tx_map.insert(i, tx);
        let syncer = s.clone();
        let h = tokio::spawn(save_task(
            store_typ.clone(),
            i,
            syncer,
            rx,
            shutdown_tx.subscribe(),
        ));
        fut.push(h);
    }
    let syncer_len = syncer.len();
    log::info!("syncer counts: {}", syncer_len);

    let mut task_n = 0;
    let len = syncer_len / task_count;
    let len_n = len * task_count;
    let mut sub_types = Vec::new();
    for i in 0..syncer_len {
        sub_types.push(i);
        if i + 1 >= len_n {
            continue;
        }
        if sub_types.len() >= len {
            task_n += 1;
            log::info!("start fetch task#{}", task_n);
            let h = tokio::spawn(fetch_task(
                sub_types,
                store.clone(),
                tx_map.clone(),
                shutdown_tx.subscribe(),
            ));
            fut.push(h);
            sub_types = Vec::new();
        }
    }
    if sub_types.len() > 0 {
        task_n += 1;
        log::info!("start fetch task#{}", task_n);
        let h = tokio::spawn(fetch_task(
            sub_types,
            store.clone(),
            tx_map,
            shutdown_tx.subscribe(),
        ));
        fut.push(h);
    }

    tokio::select! {
        _ = join_all(fut) => {},
        _ = shutdown_rx.recv() => {
            log::info!("sync_task shutdown recv");
            shutdown_tx.send(()).map_err(|e| {
                log::error!("sync_task shutdown send error: {:?}", e);
                Error::Custom(format!("sync_task shutdown send error: {:?}", e))
            })?;
        }
    }

    Ok(())
}

async fn fetch_task(
    syncer_index: Vec<usize>,
    store: Arc<Box<dyn Store>>,
    tx_map: HashMap<usize, UnboundedSender<SyncData>>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    let syncer = store.syncer()?;
    for index in syncer_index.into_iter() {
        let s = syncer.get(index).unwrap();
        let tx = tx_map.get(&index).unwrap();
        tokio::select! {
            _ = s.fetch(tx.clone()) => {},
            _ = shutdown_rx.recv() => {
                log::info!("fetch_task shutdown recv");
                break;
            }
        }
        tx.send(SyncData::Done).map_err(|e| {
            log::error!("send data error {:?}", e);
            Error::Custom(format!("send data error {:?}", e))
        })?;
    }
    Ok(())
}

async fn save_task(
    store_typ: SyncDestType,
    index: usize,
    syncer: Arc<Box<dyn Syncer>>,
    mut rx: mpsc::UnboundedReceiver<SyncData>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    log::info!("store({:?})#{} save task start", &store_typ, index);

    loop {
        let data = rx.recv().await;
        if let Some(d) = data {
            if matches!(d, SyncData::Done) {
                break;
            }
            tokio::select! {
                _ = syncer.save(d) => {},
                _ = shutdown_rx.recv() => {
                    log::info!("save_task shutdown recv");
                    break;
                }
            }
        } else {
            break;
        }
    }

    log::info!("store({:?})#{} save task done", &store_typ, index);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{SyncDataType, SyncDest};

    use super::Sync;
    use tokio::sync::broadcast;

    #[test]
    fn test() {
        fern::Dispatch::new()
            .filter(|f| f.target().starts_with("rwqdata"))
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout())
            .apply()
            .unwrap();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let (_, rx) = broadcast::channel(1);
                let mut s = Sync::new(
                    vec![
                        SyncDest::MongoDB("mongodb://localhost:27017".into()),
                    ],
                    rx,
                    // None,
                    Some(vec![SyncDataType::StockMargin]),
                );
                s.sync(true, 1, 1).await.unwrap();
            });
    }
}
