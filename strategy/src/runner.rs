use std::{collections::HashMap, sync::Arc};

use bson::doc;
use futures::future::{join, join_all};
use hiq_data::store::Loader;

use crate::{Error, ProgressFunc, Result, Strategy, StrategyResult, StrategyType};
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinHandle,
};

pub fn strategy_to_data_type(typ: StrategyType) -> hiq_data::store::DataType {
    match typ {
        StrategyType::Bond => hiq_data::store::DataType::Bond,
        StrategyType::Fund => hiq_data::store::DataType::Fund,
        StrategyType::Stock => hiq_data::store::DataType::Stock,
        StrategyType::Index => hiq_data::store::DataType::Index,
        StrategyType::Concept => hiq_data::store::DataType::Concept,
        StrategyType::Industry => hiq_data::store::DataType::Industry,
    }
}

pub async fn run(
    strategy: Arc<Box<dyn Strategy>>,
    loader: Arc<Box<dyn Loader>>,
    concurrent: usize,
    mut shutdown_rx: broadcast::Receiver<()>,
    the_codes: Option<HashMap<StrategyType, Vec<(String, String)>>>,
    progress_func: Option<ProgressFunc>,
) -> Result<Option<HashMap<StrategyType, Vec<StrategyResult>>>> {
    let (shutdown_tx, _) = broadcast::channel(1);
    let types = strategy.accept();
    let mut test_codes = HashMap::new();
    let mut total = 0;
    if let Some(the_codes) = the_codes {
        for (k, v) in the_codes.into_iter() {
            total += v.len();
            let codes = split_code(v, concurrent);
            test_codes.insert(k, codes);
        }
    } else {
        for typ in types.into_iter() {
            let codes = loader
                .load_info(strategy_to_data_type(typ), doc! {}, doc! {}, None)
                .await
                .map_err(|e| Error::Custom(format!("query info error: {:?}", e)))?;

            total += codes.len();

            let codes = split_code(codes, concurrent);

            if !codes.is_empty() {
                test_codes.insert(typ, codes);
            }
        }
    }

    let (progress_tx, progress_rx, progress_func) = if let Some(progress_func) = progress_func {
        let (progress_tx, progress_rx) = mpsc::unbounded_channel();

        let progress_func = Arc::new(progress_func);

        (Some(progress_tx), Some(progress_rx), Some(progress_func))
    } else {
        (None, None, None)
    };

    let mut handlers = HashMap::new();
    for (typ, codes_vec) in test_codes.into_iter() {
        let mut handler = Vec::new();
        for codes in codes_vec.into_iter() {
            let s = strategy.clone();
            let l = loader.clone();
            let rx = shutdown_tx.subscribe();
            log::info!("spawn task: {:?}", &typ);
            let tx = progress_tx.clone();
            let h = tokio::spawn(run_task(typ, s, l, codes, rx, tx));
            handler.push(h);
        }
        handlers.insert(typ, handler);
    }
    let mut g_handlers = Vec::new();
    for (typ, handlers) in handlers.into_iter() {
        let h = tokio::spawn(join_group(typ, handlers));
        g_handlers.push(h);
    }
    let mut ret_map = HashMap::new();

    let p_handler = tokio::spawn(progress_task(
        total,
        progress_func,
        shutdown_tx.subscribe(),
        progress_rx,
    ));

    tokio::select! {
        (rest, _) = join(join_all(g_handlers), p_handler) => {
            log::info!("join_all done");
            for res in rest.into_iter() {
                let res = res.map_err(|e| Error::Custom(format!("join error: {}", e.to_string())))?;
                if let Ok(res) = res {
                    if let Some((typ, data)) = res {
                        ret_map.insert(typ, data);
                    }
                }
            }

        },
        _ = shutdown_rx.recv() => {
            log::info!("receive shutdown signal");
            shutdown_tx.send(())
            .map_err(|e|Error::Custom(format!("send shutdown signal error x: {}", e.to_string())))?;
        }
    }

    let ret_map = if ret_map.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ret_map))
    };
    log::info!("all task done");
    ret_map
}

pub async fn fit(
    code: String,
    name: String,
    typ: StrategyType,
    strategy: Arc<Box<dyn Strategy>>,
    loader: Arc<Box<dyn Loader>>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<Option<StrategyResult>> {
    let types = strategy.accept();
    if !types.contains(&typ) {
        return Err(Error::Custom(format!(
            "strategy not suitable for type: {:?}, only valid for {:?}",
            &typ, types
        )));
    }
    tokio::select! {
        data = strategy.test(loader.clone(), typ, code, name) => {
            return data;
        },
        _ = shutdown_rx.recv() => {
            log::info!("receive shutdown signal");
            return Ok(None);
        }
    }
}

async fn join_group(
    typ: StrategyType,
    handlers: Vec<JoinHandle<Result<Option<Vec<StrategyResult>>>>>,
) -> Result<Option<(StrategyType, Vec<StrategyResult>)>> {
    let rest = join_all(handlers).await;
    let mut rs_vec = Vec::new();
    for res in rest.into_iter() {
        let res = res.map_err(|e| Error::Custom(format!("join error: {}", e.to_string())))?;
        if res.is_err() {
            log::error!("task run error: {:?}", res);
            continue;
        }
        if let Some(res) = res.unwrap() {
            rs_vec.extend(res.into_iter());
        }
    }
    let rs = if rs_vec.is_empty() {
        Ok(None)
    } else {
        Ok(Some((typ.clone(), rs_vec)))
    };
    log::info!("type {:?} all task done", typ);
    rs
}

async fn progress_task(
    total: usize,
    progress_func: Option<Arc<ProgressFunc>>,
    mut shutdown_rx: broadcast::Receiver<()>,
    progress_rx: Option<mpsc::UnboundedReceiver<(String, String)>>,
) {
    if progress_func.is_none() || progress_rx.is_none() {
        return;
    }
    let progress_func = progress_func.unwrap();
    let mut progress_rx = progress_rx.unwrap();

    log::info!("progress task with {} codes", total);
    let mut current: usize = 0;
    for _ in 0..total {
        tokio::select! {
            rs = progress_rx.recv() => {
                current+=1;
                if let Some(rs) = rs {
                    let p = (((current * 100) as f32 / total as f32) * 100.0).round() / 100.0;
                    let (code, name) = rs;
                    progress_func(code.as_str(), name.as_str(), total, current, p);
                } else {
                    log::error!("progress_rx receive None");
                    break;
                }

            }
            _ = shutdown_rx.recv() => {
                log::info!("run task receive shutdown signal");
                break;
            }
        }
    }
}

async fn run_task(
    typ: StrategyType,
    strategy: Arc<Box<dyn Strategy>>,
    loader: Arc<Box<dyn Loader>>,
    codes: Vec<(String, String)>,
    mut shutdown_rx: broadcast::Receiver<()>,
    progress_tx: Option<mpsc::UnboundedSender<(String, String)>>,
) -> Result<Option<Vec<StrategyResult>>> {
    log::info!("run task with {} codes", codes.len());
    let mut rs_vec = Vec::new();
    for (code, name) in codes {
        if let Some(tx) = &progress_tx {
            tx.send((code.clone(), name.clone())).map_err(|e| {
                Error::Custom(format!("send progress signal error x: {}", e.to_string()))
            })?;
        }
        tokio::select! {
            res = strategy
            .test(loader.clone(), typ, code.clone(), name.clone()) => {
                if res.is_err() {
                    log::error!("run test with error: {:?}", res);
                    continue;
                }
                let res = res.unwrap();
                if let Some(data) = res {
                    log::info!("got data: {}({})", name, code);
                    rs_vec.push(data);
                }
            },
            _ = shutdown_rx.recv() => {
                log::info!("run task receive shutdown signal");
                return Ok(None);
            }
        }
    }

    if rs_vec.is_empty() {
        Ok(None)
    } else {
        Ok(Some(rs_vec))
    }
}

fn split_code(codes: Vec<(String, String)>, count: usize) -> Vec<Vec<(String, String)>> {
    let len = codes.len();
    let task_count = len / count;

    let mut result = Vec::new();
    let mut task_vec = Vec::new();
    for code in codes.into_iter() {
        task_vec.push(code);
        if task_vec.len() < task_count {
            continue;
        }
        result.push(task_vec);
        task_vec = Vec::new();
    }
    if task_vec.len() > 0 {
        result.push(task_vec);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use hiq_data::{
        store::{get_loader, Loader},
        HiqSyncDest,
    };
    use tokio::sync::broadcast;

    use crate::{run, Result, Strategy, StrategyResult, StrategyType};

    use async_trait::async_trait;

    struct TestStrategy {}
    #[async_trait]
    impl Strategy for TestStrategy {
        async fn test(
            &self,
            _loader: Arc<Box<dyn Loader>>,
            _typ: StrategyType,
            code: String,
            name: String,
        ) -> Result<Option<StrategyResult>> {
            let codes = vec![
                "sz002805".to_string(),
                "sz300827".to_string(),
                "sz000762".to_string(),
            ];
            if codes.contains(&code) {
                let rs = StrategyResult {
                    code,
                    name,
                    mark: None,
                    stat: None,
                };
                return Ok(Some(rs));
            }
            Ok(None)
        }
    }
    #[test]
    fn test_runner() {
        fern::Dispatch::new()
            // .filter(|f| f.target().starts_with("hiq"))
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Error)
            .chain(std::io::stdout())
            .apply()
            .unwrap();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                log::info!("set up logger");
                let (tx, _) = broadcast::channel(1);
                let dest = HiqSyncDest::MongoDB("mongodb://localhost:27017".to_owned());

                let (_, loader) = get_loader(&dest, true).await.unwrap();
                let mut strategy: Box<dyn Strategy> = Box::new(TestStrategy {});
                let loader = Arc::new(loader);

                strategy
                    .prepare(loader.clone(), Some(Default::default()), None)
                    .await
                    .unwrap();
                let strategy = Arc::new(strategy);
                let result = run(
                    strategy,
                    loader,
                    5,
                    tx.subscribe(),
                    None,
                    Some(Box::new(|code, name, t, c, p| {
                        print!("\rprocessing: {}({}) {}/{}({})%", name, code, c, t, p)
                    })),
                )
                .await
                .unwrap();
                log::info!("result: {:?}", result);
            });
    }
}
