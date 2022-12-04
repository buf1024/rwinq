use std::{collections::HashMap, sync::Arc};

use bson::doc;
use futures::future::join_all;
use hiq_data::store::Loader;

use crate::{Error, Result, Strategy, StrategyResult, StrategyType};
use tokio::{sync::broadcast, task::JoinHandle};

pub async fn run(
    strategy: Arc<Box<dyn Strategy>>,
    loader: Arc<Box<dyn Loader>>,
    concurrent: usize,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<Option<HashMap<StrategyType, Vec<StrategyResult>>>> {
    let (shutdown_tx, _) = broadcast::channel(1);
    let types = strategy.accept();
    let mut test_codes = HashMap::new();
    for typ in types.into_iter() {
        let codes: Vec<_> = match typ {
            StrategyType::Bond => {
                let info = loader
                    .load_bond_info(doc! {}, doc! {}, None)
                    .await
                    .map_err(|e| Error::Custom(format!("query bond info error: {:?}", e)))?;
                split_code(
                    info.into_iter().map(|e| (e.code, e.name)).collect(),
                    concurrent,
                )
            }
            StrategyType::Fund => {
                let info = loader
                    .load_fund_info(doc! {}, doc! {}, None)
                    .await
                    .map_err(|e| Error::Custom(format!("query fund info error: {:?}", e)))?;
                split_code(
                    info.into_iter().map(|e| (e.code, e.name)).collect(),
                    concurrent,
                )
            }
            StrategyType::Stock => {
                let info = loader
                    .load_stock_info(doc! {}, doc! {}, None)
                    .await
                    .map_err(|e| Error::Custom(format!("query stock info error: {:?}", e)))?;
                split_code(
                    info.into_iter().map(|e| (e.code, e.name)).collect(),
                    concurrent,
                )
            }
            StrategyType::Index => {
                let info = loader
                    .load_index_info(doc! {}, doc! {}, None)
                    .await
                    .map_err(|e| Error::Custom(format!("query index info error: {:?}", e)))?;
                split_code(
                    info.into_iter().map(|e| (e.code, e.name)).collect(),
                    concurrent,
                )
            }
            StrategyType::Concept => {
                let info = loader
                    .load_stock_info(doc! {}, doc! {}, None)
                    .await
                    .map_err(|e| Error::Custom(format!("query concept info error: {:?}", e)))?;
                split_code(
                    info.into_iter().map(|e| (e.code, e.name)).collect(),
                    concurrent,
                )
            }
            StrategyType::Industry => {
                let info = loader
                    .load_stock_info(doc! {}, doc! {}, None)
                    .await
                    .map_err(|e| Error::Custom(format!("query industry info error: {:?}", e)))?;
                split_code(
                    info.into_iter().map(|e| (e.code, e.name)).collect(),
                    concurrent,
                )
            }
        };
        if !codes.is_empty() {
            test_codes.insert(typ, codes);
        }
    }

    let mut handlers = HashMap::new();
    for (typ, codes_vec) in test_codes.into_iter() {
        let mut handler = Vec::new();
        for codes in codes_vec.into_iter() {
            let s = strategy.clone();
            let l = loader.clone();
            let rx = shutdown_tx.subscribe();
            log::info!("spawn task: {:?}", &typ);
            let h = tokio::spawn(run_task(typ, s, l, codes, rx));
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
    tokio::select! {
        rest = join_all(g_handlers) => {
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

async fn run_task(
    typ: StrategyType,
    strategy: Arc<Box<dyn Strategy>>,
    loader: Arc<Box<dyn Loader>>,
    codes: Vec<(String, String)>,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<Option<Vec<StrategyResult>>> {
    log::info!("run task with {} codes", codes.len());
    let mut rs_vec = Vec::new();
    for (code, name) in codes {
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
                    marker: None,
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
            .level(log::LevelFilter::Debug)
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

                strategy.prepare(loader.clone(), None, None).await.unwrap();
                let strategy = Arc::new(strategy);
                let result = run(strategy, loader, 5, tx.subscribe()).await.unwrap();
                log::info!("result: {:?}", result);
            });
    }
}
