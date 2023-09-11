use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pywqcmm::runtime;
use rwqdata::{SyncDataType, SyncDest};
use tokio::sync::broadcast;

#[pyclass]
pub struct Sync {
    dest: Vec<SyncDest>,
    funcs: Option<Vec<SyncDataType>>,
    shutdown_tx: broadcast::Sender<()>,
}

#[pymethods]
impl Sync {
    #[new]
    fn new(dest: Vec<(String, String)>, funcs: Option<Vec<i32>>) -> Self {
        let dest: Vec<_> = dest
            .into_iter()
            .map(|e| {
                let def = e.1.clone();
                SyncDest::try_from(e).unwrap_or(SyncDest::MongoDB(def))
            })
            .collect();
        let funcs = funcs.map(|v| {
            let v: Vec<_> = v
                .into_iter()
                .map(|e| SyncDataType::try_from(e).unwrap_or(SyncDataType::StockInfo))
                .collect();
            v
        });
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            dest,
            funcs,
            shutdown_tx,
        }
    }
    fn sync<'a>(
        &self,
        py: Python<'a>,
        skip_basic: bool,
        task_count: usize,
        split_count: usize,
    ) -> PyResult<&'a PyAny> {
        let dest = self.dest.clone();
        let funcs = self.funcs.clone();
        let shutdown_rx = self.shutdown_tx.subscribe();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut rwqsync = rwqdata::Sync::new(dest, shutdown_rx, funcs);
            Ok(rwqsync
                .sync(skip_basic, task_count, split_count)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?)
        })
    }
    fn shutdown(&self) -> PyResult<()> {
        self.shutdown_tx
            .send(())
            .map_err(|e| PyException::new_err(e.to_string()))?;
        Ok(())
    }
}

#[pyclass]
pub(crate) struct BlockSync {
    dest: Vec<SyncDest>,
    funcs: Option<Vec<SyncDataType>>,
    shutdown_tx: broadcast::Sender<()>,
}

#[pymethods]
impl BlockSync {
    #[new]
    fn new(dest: Vec<(String, String)>, funcs: Option<Vec<i32>>) -> Self {
        let dest: Vec<_> = dest
            .into_iter()
            .map(|e| {
                let def = e.1.clone();
                SyncDest::try_from(e).unwrap_or(SyncDest::MongoDB(def))
            })
            .collect();
        let funcs = funcs.map(|v| {
            let v: Vec<_> = v
                .into_iter()
                .map(|e| SyncDataType::try_from(e).unwrap_or(SyncDataType::StockInfo))
                .collect();
            v
        });
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            dest,
            funcs,
            shutdown_tx,
        }
    }
    fn sync(&self, skip_basic: bool, task_count: usize, split_count: usize) -> PyResult<()> {
        let dest = self.dest.clone();
        let funcs = self.funcs.clone();
        let shutdown_rx = self.shutdown_tx.subscribe();
        Ok(runtime()?.block_on(block_sync(
            dest,
            shutdown_rx,
            funcs,
            skip_basic,
            task_count,
            split_count,
        ))?)
    }
    fn shutdown(&self) -> PyResult<()> {
        self.shutdown_tx
            .send(())
            .map_err(|e| PyException::new_err(e.to_string()))?;
        Ok(())
    }
}

async fn block_sync(
    dest: Vec<SyncDest>,
    shutdown_rx: broadcast::Receiver<()>,
    funcs: Option<Vec<SyncDataType>>,
    skip_basic: bool,
    task_count: usize,
    split_count: usize,
) -> PyResult<()> {
    let mut rwqsync = rwqdata::Sync::new(dest, shutdown_rx, funcs);

    rwqsync
        .sync(skip_basic, task_count, split_count)
        .await
        .map_err(|e| PyException::new_err(e.to_string()))
}
