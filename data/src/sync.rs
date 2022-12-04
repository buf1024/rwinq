use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;

use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{broadcast, mpsc};

use crate::store::get_store;
use crate::types::HiqSyncDataType;
use crate::{
    store::Store,
    syncer::Syncer,
    types::{HiqSyncData, HiqSyncDest, HiqSyncDestType},
    Error, Result,
};

pub struct HiqSync {
    dest: Vec<HiqSyncDest>,
    shutdown: broadcast::Receiver<()>,
    funcs: Option<Vec<HiqSyncDataType>>,
    store: Option<HashMap<HiqSyncDestType, Arc<Box<dyn Store>>>>,
    is_init: bool,
}

impl HiqSync {
    pub fn new(
        dest: Vec<HiqSyncDest>,
        shutdown: broadcast::Receiver<()>,
        funcs: Option<Vec<HiqSyncDataType>>,
    ) -> Self {
        Self {
            dest,
            shutdown,
            funcs,
            store: None,
            is_init: false,
        }
    }
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
                        Error::Custom("shutdown send error")
                    })?;
                }
            }
        }
        log::info!("done sync");
        Ok(())
    }
}

async fn sync_task(
    store_typ: HiqSyncDestType,
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
                log::error!("send data error {:?}", e);
                Error::Custom("sync_task shutdown send error")
            })?;
        }
    }

    Ok(())
}

async fn fetch_task(
    syncer_index: Vec<usize>,
    store: Arc<Box<dyn Store>>,
    tx_map: HashMap<usize, UnboundedSender<HiqSyncData>>,
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
        tx.send(HiqSyncData::Done).map_err(|e| {
            log::error!("send data error {:?}", e);
            Error::Custom("queue send error")
        })?;
    }
    Ok(())
}

async fn save_task(
    store_typ: HiqSyncDestType,
    index: usize,
    syncer: Arc<Box<dyn Syncer>>,
    mut rx: mpsc::UnboundedReceiver<HiqSyncData>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    log::info!("store({:?})#{} save task start", &store_typ, index);

    loop {
        let data = rx.recv().await;
        if let Some(d) = data {
            if matches!(d, HiqSyncData::Done) {
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
    use crate::types::{HiqSyncDataType, HiqSyncDest};

    use super::HiqSync;
    use tokio::sync::broadcast;

    #[test]
    fn test() {
        fern::Dispatch::new()
            .filter(|f| f.target().starts_with("hiq_data"))
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
                let mut s = HiqSync::new(
                    vec![
                        HiqSyncDest::MongoDB("mongodb://localhost:27017".into()),
                        // HiqSyncDest::ClickHouse("abc".into()),
                    ],
                    rx,
                    // None,
                    Some(vec![HiqSyncDataType::StockMargin]),
                );
                s.sync(true, 1, 1).await.unwrap();
            });
    }
}
