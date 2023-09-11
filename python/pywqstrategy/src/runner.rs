use std::{collections::HashMap, sync::Arc};

use pyo3::{
    exceptions::PyException,
    prelude::*,
    types::{PyDict, PyList, PyTuple},
};
use rwqstrategy::{
    store::{get_loader, Loader},
    CommonParam, ProgressFunc, Strategy, StrategyResult, StrategyType, SyncDest,
};

use async_trait::async_trait;
use pyo3_asyncio::TaskLocals;
use tokio::sync::broadcast;

async fn prepare_runner(typ: String, url: String) -> PyResult<Arc<Box<dyn Loader>>> {
    let dest = SyncDest::try_from((typ, url)).map_err(|e| PyException::new_err(e.to_string()))?;
    let (_, loader) = get_loader(&dest, true)
        .await
        .map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(Arc::new(loader))
}

struct WrapStrategy {
    prepare: Py<PyAny>,
    run: Py<PyAny>,
    locals: TaskLocals,
}
#[async_trait]
impl Strategy for WrapStrategy {
    async fn prepare(
        &mut self,
        _: Arc<Box<dyn Loader>>,
        _: Option<CommonParam>,
        _: Option<HashMap<String, String>>,
    ) -> rwqstrategy::Result<()> {
        let locals = self.locals.clone();
        let res = Python::with_gil(|py| {
            let coroutine = self.prepare.call0(py)?;
            pyo3_asyncio::into_future_with_locals(&locals, coroutine.into_ref(py))
        })
        .map_err(|e| {
            rwqstrategy::Error::Custom(format!("call python strategy prepare error: {:?}", e))
        })?
        .await
        .map_err(|e| {
            rwqstrategy::Error::Custom(format!("await python strategy prepare error: {:?}", e))
        })?;
        let res: bool = Python::with_gil(|py| res.extract(py)).map_err(|e| {
            rwqstrategy::Error::Custom(format!("extract return value error: {:?}", e))
        })?;
        log::info!("prepare return: {}", res);
        if res {
            Ok(())
        } else {
            Err(rwqstrategy::Error::Custom(format!(
                "call prepare return false"
            )))
        }
    }
    async fn test(
        &self,
        _: Arc<Box<dyn Loader>>,
        typ: StrategyType,
        code: String,
        name: String,
    ) -> rwqstrategy::Result<Option<StrategyResult>> {
        let locals = self.locals.clone();
        let res = Python::with_gil(|py| {
            let typ = crate::types::StrategyType::from(typ);
            let args = PyTuple::new(py, &[typ.into_py(py), code.into_py(py), name.into_py(py)]);
            let coroutine = self.run.call1(py, args)?;
            pyo3_asyncio::into_future_with_locals(&locals, coroutine.into_ref(py))
        })
        .map_err(|e| {
            rwqstrategy::Error::Custom(format!("call python strategy test error: {:?}", e))
        })?
        .await
        .map_err(|e| {
            rwqstrategy::Error::Custom(format!("await python strategy test error: {:?}", e))
        })?;

        let res: Option<StrategyResult> = Python::with_gil(|py| {
            if res.is_none(py) {
                Ok(None)
            } else {
                let s: String = res.extract(py)?;
                let s: StrategyResult =
                    serde_json::from_str(&s).map_err(|e| PyException::new_err(e.to_string()))?;
                log::info!("got data: {:?}", &s);
                Ok(Some(s))
            }
        })
        .map_err(|e: PyErr| {
            rwqstrategy::Error::Custom(format!("extract return value error: {:?}", e))
        })?;

        Ok(res)
    }
}

fn extract(
    the_codes: Option<PyObject>,
) -> PyResult<Option<HashMap<StrategyType, Vec<(String, String)>>>> {
    if let Some(the_codes) = the_codes {
        Python::with_gil(|py| {
            let mut map = HashMap::new();
            let py_dict: &PyDict = the_codes.downcast(py)?;
            for (k, v) in py_dict.into_iter() {
                let typ: i32 = k.extract()?;
                let py_list: &PyList = v.downcast()?;
                let mut items = Vec::new();
                for pair in py_list.into_iter() {
                    let py_tuple: &PyTuple = pair.downcast()?;
                    if py_tuple.len() != 2 {
                        return Err(PyException::new_err(
                            "code name pair not correct".to_owned(),
                        ));
                    }
                    let code: String = py_tuple.get_item(0)?.extract()?;
                    let name: String = py_tuple.get_item(1)?.extract()?;
                    items.push((code, name));
                }

                map.insert(StrategyType::from(typ), items);
            }
            Ok(Some(map))
        })
    } else {
        Ok(None)
    }
}

fn progress_callback(py_callback: Option<PyObject>) -> Option<ProgressFunc> {
    if py_callback.is_none() {
        return None;
    }
    let py_callback = py_callback.unwrap();
    return Some(Box::new(
        move |code: &str, name: &str, total: usize, current: usize, progress: f32| {
            Python::with_gil(|py| {
                let args = PyTuple::new(
                    py,
                    &[
                        code.into_py(py),
                        name.into_py(py),
                        total.into_py(py),
                        current.into_py(py),
                        progress.into_py(py),
                    ],
                );
                let r = py_callback.call1(py, args);
                if r.is_err() {
                    log::error!("call python process callback error: {:?}", r);
                }
            })
        },
    ));
}

#[pyclass]
pub(crate) struct Runner {
    loader: Arc<Box<dyn Loader>>,
    concurrent: usize,
    shutdown_tx: broadcast::Sender<()>,
}

// 直接传 PyObject

#[pymethods]
impl Runner {
    #[new]
    fn new<'a>(py: Python<'a>, typ: String, url: String, concurrent: usize) -> PyResult<Self> {
        let event_loop = py.import("asyncio")?.call_method0("new_event_loop")?;
        let loader = pyo3_asyncio::tokio::run_until_complete(event_loop, prepare_runner(typ, url))?;
        let (shutdown_tx, _) = broadcast::channel(1);
        Ok(Self {
            loader,
            concurrent,
            shutdown_tx,
        })
    }
    /// 运行python策略
    fn run<'a>(
        &self,
        py: Python<'a>,
        py_strategy: PyObject,
        the_codes: Option<PyObject>,
        py_callback: Option<PyObject>,
    ) -> PyResult<&'a PyAny> {
        let loader = self.loader.clone();
        let concurrent = self.concurrent;
        let shutdown_rx = self.shutdown_tx.subscribe();
        let prepare = py_strategy.getattr(py, "prepare")?;
        let run = py_strategy.getattr(py, "run")?;

        let locals = pyo3_asyncio::tokio::get_current_locals(py)?;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let the_codes = extract(the_codes)?;
            log::info!("arguments: {:?}", the_codes);
            let mut strategy = WrapStrategy {
                prepare,
                run,
                locals,
            };
            strategy
                .prepare(loader.clone(), None, None)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?;

            let strategy: Arc<Box<dyn Strategy>> = Arc::new(Box::new(strategy));

            log::info!("start run strategy");
            let data = rwqstrategy::run(
                strategy,
                loader,
                concurrent,
                shutdown_rx,
                the_codes,
                progress_callback(py_callback),
            )
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?;
            log::info!("done run strategy");
            if let Some(data) = data {
                let m: HashMap<_, _> = data
                    .into_iter()
                    .map(|(k, v)| {
                        let key = crate::types::StrategyType::from(k);
                        let val: Vec<_> = v
                            .into_iter()
                            .map(crate::types::StrategyResult::from)
                            .collect();
                        (key, val)
                    })
                    .collect();
                Ok(Some(m))
            } else {
                Ok(None)
            }
        })
    }

    /// 运行测试某是否符合python策略
    fn fit<'a>(
        &self,
        py: Python<'a>,
        py_strategy: PyObject,
        code: String,
        name: String,
        typ: i32,
    ) -> PyResult<&'a PyAny> {
        let loader = self.loader.clone();
        let shutdown_rx = self.shutdown_tx.subscribe();
        let prepare = py_strategy.getattr(py, "prepare")?;
        let run = py_strategy.getattr(py, "run")?;
        let locals = pyo3_asyncio::tokio::get_current_locals(py)?;
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let typ = StrategyType::from(typ);
            log::info!("arguments: code={}, name={}, typ={:?}", &code, &name, &typ);
            let mut strategy = WrapStrategy {
                prepare,
                run,
                locals,
            };
            strategy
                .prepare(loader.clone(), None, None)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?;

            let strategy: Arc<Box<dyn Strategy>> = Arc::new(Box::new(strategy));

            log::info!("start strategy fit test");
            let data = rwqstrategy::fit(code, name, typ, strategy, loader, shutdown_rx)
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?;
            log::info!("done strategy fit test");
            if let Some(data) = data {
                let m = crate::types::StrategyResult::from(data);
                Ok(Some(m))
            } else {
                Ok(None)
            }
        })
    }

    fn shutdown(&self) -> PyResult<bool> {
        self.shutdown_tx
            .send(())
            .map_err(|e| PyException::new_err(e.to_string()))?;
        Ok(true)
    }
}
