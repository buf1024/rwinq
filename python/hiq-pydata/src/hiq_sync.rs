use hiq_data::{HiqSyncDataType, HiqSyncDest};
use hiq_pycommon::runtime;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use tokio::sync::broadcast;

#[pyclass]
pub struct HiqSync {
    dest: Vec<HiqSyncDest>,
    funcs: Option<Vec<HiqSyncDataType>>,
    shutdown_tx: broadcast::Sender<()>,
}

#[pymethods]
impl HiqSync {
    #[new]
    fn new(dest: Vec<(String, String)>, funcs: Option<Vec<i32>>) -> Self {
        let dest: Vec<_> = dest
            .into_iter()
            .map(|e| {
                let def = e.1.clone();
                HiqSyncDest::try_from(e).unwrap_or(HiqSyncDest::MongoDB(def))
            })
            .collect();
        let funcs = funcs.map(|v| {
            let v: Vec<_> = v
                .into_iter()
                .map(|e| HiqSyncDataType::try_from(e).unwrap_or(HiqSyncDataType::StockInfo))
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
    fn sync<'a>(&self, py: Python<'a>, skip_basic: bool, task_count: usize, split_count: usize, ) -> PyResult<&'a PyAny> {
        let dest = self.dest.clone();
        let funcs = self.funcs.clone();
        let shutdown_rx = self.shutdown_tx.subscribe();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut hiq_sync = hiq_data::HiqSync::new(dest, shutdown_rx, funcs);
            Ok(hiq_sync
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
pub(crate) struct BlockHiqSync {
    dest: Vec<HiqSyncDest>,
    funcs: Option<Vec<HiqSyncDataType>>,
    shutdown_tx: broadcast::Sender<()>,
}

#[pymethods]
impl BlockHiqSync {
    #[new]
    fn new(dest: Vec<(String, String)>, funcs: Option<Vec<i32>>) -> Self {
        let dest: Vec<_> = dest
            .into_iter()
            .map(|e| {
                let def = e.1.clone();
                HiqSyncDest::try_from(e).unwrap_or(HiqSyncDest::MongoDB(def))
            })
            .collect();
        let funcs = funcs.map(|v| {
            let v: Vec<_> = v
                .into_iter()
                .map(|e| HiqSyncDataType::try_from(e).unwrap_or(HiqSyncDataType::StockInfo))
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
        Ok(runtime()?.block_on(block_sync(dest, shutdown_rx, funcs, skip_basic, task_count, split_count))?)
    }
    fn shutdown(&self) -> PyResult<()> {
        self.shutdown_tx
            .send(())
            .map_err(|e| PyException::new_err(e.to_string()))?;
        Ok(())
    }
}

async fn block_sync(
    dest: Vec<HiqSyncDest>,
    shutdown_rx: broadcast::Receiver<()>,
    funcs: Option<Vec<HiqSyncDataType>>,
    skip_basic: bool,
    task_count: usize,
    split_count: usize,
) -> PyResult<()> {
    let mut hiq_sync = hiq_data::HiqSync::new(dest, shutdown_rx, funcs);

    hiq_sync
        .sync(skip_basic, task_count, split_count)
        .await
        .map_err(|e| PyException::new_err(e.to_string()))
}
