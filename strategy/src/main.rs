use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Context;
use argh::FromArgs;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use hiq_data::{
    store::{get_loader, Loader},
    HiqSyncDest,
};
use hiq_strategy::{
    fit, get_strategy, run, strategies, CommonParam, ProgressFunc, Strategy, StrategyType, Symbol,
    SYMBOL_NAME,
};
use tokio::{signal, sync::broadcast};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let s: HiqStrategy = argh::from_env();
    if s.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if let Some(h) = s.cmd {
        match h {
            HiqHelpCommandEnum::Help(s) => {
                if let Some(path) = &s.path {
                    let res = build_dll_strategy(path);
                    if res.is_err() {
                        println!("build_dll_strategy error: {:?}", res.err());
                        return Ok(());
                    }
                    let strategy = res.unwrap();
                    println!(
                        "dll strategy: {}\nusage:\n{}",
                        strategy.name(),
                        strategy.help()
                    );
                }
                if let Some(name) = &s.builtin {
                    let strategy = get_strategy(name);
                    if strategy.is_err() {
                        println!("builtin strategy {} not found", name);
                        return Ok(());
                    }
                    let strategy = strategy.unwrap();
                    println!(
                        "builtin strategy: {}\nusage:\n{}",
                        strategy.name(),
                        strategy.help()
                    );
                }
                if s.path.is_none() && s.builtin.is_none() {
                    let s = strategies();
                    for name in s {
                        let strategy = get_strategy(&name);
                        if strategy.is_err() {
                            println!("builtin strategy {} not found", name);
                            return Ok(());
                        }
                        let strategy = strategy.unwrap();
                        println!(
                            "builtin strategy: {}\nusage:\n{}",
                            strategy.name(),
                            strategy.help()
                        );
                    }
                }
            }
        }

        return Ok(());
    }

    let res = set_logger(&s.level);
    if res.is_err() {
        println!("set up logger error: {:?}", res);
        return Ok(());
    }
    log::info!("logger is ready");

    let rs = build_app_params(&s.dest, &s.params).await;
    if rs.is_err() {
        println!("build_app_params error: {:?}", rs.err());
        return Ok(());
    }
    let (loader, cmm_params, params) = rs.unwrap();

    let (shutdown_tx, _) = broadcast::channel(1);
    let mut strategy = if let Some(path) = &s.path {
        let res = build_dll_strategy(path);
        if res.is_err() {
            println!("build_dll_strategy error: {:?}", res.err());
            return Ok(());
        }
        res.unwrap()
    } else {
        if s.builtin.is_none() {
            return Ok(());
        }
        let name = s.builtin.unwrap();

        let strategy = get_strategy(&name);
        if strategy.is_err() {
            println!("get_strategy strategy {} err: {:?}", &name, strategy.err());
            return Ok(());
        }
        strategy.unwrap()
    };
    strategy
        .prepare(loader.clone(), cmm_params.clone(), params.clone())
        .await?;

    if s.function == "run".to_string() {
        let func: Option<ProgressFunc> = if s.level.as_str() == "error" {
            Some(Box::new(progress))
        } else {
            None
        };
        tokio::select! {
            rs = run(
                Arc::new(strategy),
                loader,
                s.concurrent,
                shutdown_tx.subscribe(),
                None,
                func
            ) => {
                if rs.is_err() {
                    println!("run strategy error: {:?}", rs.err());
                    return Ok(());
                }
                let rs = rs.unwrap();
                if let Some(rs) = rs {
                    println!("\nrun strategy result:");
                    for (k, v) in rs {
                        println!("type: {:?}", k);
                        println!("result: {:?}", v);
                    }
                } else {
                    println!("\nrun strategy with no result!");
                }
            },
            _ = signal::ctrl_c() => {
                log::info!("shutdown receive");
                let _ = shutdown_tx.send(());
            }
        }
    } else {
        if s.code.is_none() {
            println!("test strategy need code!");
            return Ok(());
        }
        let code = s.code.unwrap();
        let typ = if let Some(typ) = &s.typ {
            StrategyType::from(typ.as_str())
        } else {
            StrategyType::Stock
        };
        tokio::select! {
            rs = fit(
                code,
                String::from(""),
                typ,
                Arc::new(strategy),
                loader,
                shutdown_tx.subscribe(),
            ) => {
                if rs.is_err() {
                    println!("test strategy fit error: {:?}", rs.err());
                    return Ok(());
                }
                let rs = rs.unwrap();
                if let Some(rs) = rs {
                    println!("test strategy fit result: {:?}", rs);
                } else {
                    println!("test strategy fit with no result!");
                }
            },
            _ = signal::ctrl_c() => {
                log::info!("shutdown receive");
                let _ = shutdown_tx.send(());
            }
        }
    }

    Ok(())
}

fn set_logger(level: &str) -> anyhow::Result<()> {
    let level_str = level.to_uppercase();
    let level = log::LevelFilter::from_str(level_str.as_str())
        .with_context(|| format!("invalid log level {}", level_str))?;
    fern::Dispatch::new()
        .filter(|f| f.target().starts_with("hiq"))
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.3f]"),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn build_dll_strategy(path: &str) -> anyhow::Result<Box<dyn Strategy>> {
    unsafe {
        let lib = libloading::Library::new(path)
            .with_context(|| format!("failed to load dll, path: {}", path))?;
        let func: libloading::Symbol<Symbol> = lib
            .get(SYMBOL_NAME.as_bytes())
            .with_context(|| format!("failed to get dll function name, path: {}", path))?;
        let raw = func() as *mut Box<dyn Strategy>;
        let strategy = *Box::from_raw(raw);
        Ok(strategy)
    }
}

async fn build_loader(org_s: &str) -> anyhow::Result<Arc<Box<dyn Loader>>> {
    let s: Vec<_> = org_s.split("=").collect();
    if s.len() != 2 {
        return Err(anyhow::anyhow!("invalid dest format"));
    }
    let (k, v) = (*s.get(0).unwrap(), *s.get(1).unwrap());
    let dest = HiqSyncDest::try_from((String::from(k), String::from(v)))
        .with_context(|| format!("try from ({}, {}) error", k, v))?;
    let (_, loader) = get_loader(&dest, true)
        .await
        .with_context(|| format!("get loader failed, params: {}", org_s))?;
    Ok(Arc::new(loader))
}

async fn build_app_params(
    dest: &String,
    args: &Vec<String>,
) -> anyhow::Result<(
    Arc<Box<dyn Loader>>,
    Option<CommonParam>,
    Option<HashMap<String, String>>,
)> {
    let loader = build_loader(dest).await?;
    let params = build_params(args)?;
    let cmm_params = build_cmm_params(&params)?;
    let params = build_strategy_params(&params);
    Ok((loader, cmm_params, params))
}

fn build_params(args: &Vec<String>) -> anyhow::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for e in args {
        let s: Vec<_> = e.split("=").collect();
        if s.len() != 2 {
            return Err(anyhow::anyhow!("invalid dest format"));
        }
        let (k, v) = (*s.get(0).unwrap(), *s.get(1).unwrap());
        if map.contains_key(k) {
            return Err(anyhow::anyhow!("duplicate param key: {}", k));
        }
        map.insert(String::from(k), String::from(v));
    }
    Ok(map)
}

fn build_cmm_params(params: &HashMap<String, String>) -> anyhow::Result<Option<CommonParam>> {
    let mut test_end_date = None;
    if params.contains_key(&String::from("test_end_date")) {
        let s = params.get(&String::from("test_end_date")).unwrap();
        let mut date = NaiveDate::parse_from_str(s, "%Y-%m-%d");
        if date.is_err() {
            date = NaiveDate::parse_from_str(s, "%Y%m%d");
            if date.is_err() {
                return Err(anyhow::anyhow!(
                    "test_end_date format is not correct, expect: %Y-%m-%d or %Y%m%d"
                ));
            }
        }
        let date = date.unwrap();
        test_end_date = Some(NaiveDateTime::new(
            date,
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        ));
    }
    let mut test_trade_days = Some(60);
    if params.contains_key(&String::from("test_trade_days")) {
        let s = params.get(&String::from("test_trade_days")).unwrap();
        let days = s.parse::<i64>();
        if days.is_err() {
            return Err(anyhow::anyhow!("test_trade_days is not number"));
        }
        let days = days.unwrap();
        test_trade_days = Some(days);
    }

    if test_end_date.is_none() || test_trade_days.is_none() {
        return Ok(None);
    }
    let cmm_param = CommonParam {
        test_end_date,
        test_trade_days,
    };
    Ok(Some(cmm_param))
}

fn build_strategy_params(params: &HashMap<String, String>) -> Option<HashMap<String, String>> {
    let mut map = HashMap::new();
    for (k, v) in params.iter() {
        if k == &String::from("test_end_date") || k == &String::from("test_trade_days") {
            continue;
        }
        map.insert((*k).clone(), (*v).clone());
    }
    if map.is_empty() {
        None
    } else {
        Some(map)
    }
}

fn progress(code: &str, name: &str, total: usize, current: usize, progress: f32) {
    print!(
        "\r>> processing: {}({}) {}/{}({})%       ",
        name, code, current, total, progress
    )
}

#[derive(FromArgs, PartialEq, Debug)]
/// HiqStrategy command.
struct HiqStrategy {
    /// 版本号
    #[argh(switch, short = 'v')]
    version: bool,

    /// 日志级别，默认info
    #[argh(option, short = 'l', default = "String::from(\"error\")")]
    level: String,

    /// 并发获取数据任务数，默认为1
    #[argh(option, short = 'r', default = "4")]
    concurrent: usize,

    /// 如：file=/user/home/app, mongodb=mongodb://localhost:27017
    /// 支持的目标有: file, mongodb, mysql, clickhouse
    /// 可同时传递多个目标:
    /// 如：-d mongodb=mongodb://localhost:27017
    #[argh(
        option,
        short = 'd',
        default = "String::from(\"mongodb=mongodb://localhost:27017\")"
    )]
    dest: String,

    /// 策略位置
    #[argh(option, short = 'p')]
    path: Option<String>,

    /// 内置策略名称
    #[argh(option, short = 'b')]
    builtin: Option<String>,

    /// 测试或者运行策略，run/test
    #[argh(option, short = 'f', default = "String::from(\"run\")")]
    function: String,

    /// function == test 有效，测试的类型
    /// 可选为：bond,fund,stock,index,concept,industry
    #[argh(option, short = 't')]
    typ: Option<String>,

    /// function == test 有效，测试的代码
    #[argh(option, short = 'c')]
    code: Option<String>,

    /// 策略参数，key=val格式
    #[argh(positional)]
    params: Vec<String>,

    #[argh(subcommand)]
    cmd: Option<HiqHelpCommandEnum>,
}
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum HiqHelpCommandEnum {
    Help(HelpCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// 策略帮助信息
#[argh(subcommand, name = "usage")]
struct HelpCommand {
    /// 策略位置
    #[argh(option, short = 'p')]
    path: Option<String>,

    /// 内置策略名称
    #[argh(option, short = 'b')]
    builtin: Option<String>,
}
