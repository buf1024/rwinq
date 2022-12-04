use anyhow::Context;
use argh::FromArgs;
use hiq_data::{HiqSync, HiqSyncDataType, HiqSyncDest};
use std::str::FromStr;

use tokio::{signal, sync::broadcast};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let s: HiqDataSync = argh::from_env();
    if s.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let res = set_logger(&s.level);
    if res.is_err() {
        println!("set up logger error: {:?}", res);
        return res;
    }
    log::info!("logger is ready");
    if let Some(cmd) = s.cmd {
        let res = match cmd {
            HiqDataSubCommandEnum::Sync(x) => sync_cmd(x).await,
            HiqDataSubCommandEnum::Build(x) => build_index(x).await,
        };
        if res.is_err() {
            log::error!("run cmd error: {:?}", res);
            return res;
        }
    }
    Ok(())
}

async fn my_exit() -> anyhow::Result<()> {
    use std::time::{Duration, Instant};

    const TIMEOUT: Duration = Duration::from_secs(3);
    let mut is_press: Option<bool> = None;
    loop {
        let now = Instant::now();
        signal::ctrl_c().await?;
        match is_press {
            Some(_) if now.elapsed() > TIMEOUT => {
                log::debug!("some press once more to exit");
            }
            None => {
                log::debug!("press once more to exit");
                is_press = Some(true);
            }
            _ => {
                log::debug!("now exit");
                break;
            }
        }
    }

    Ok(())
}

async fn build_index(cmd: BuildIndexCommand) -> anyhow::Result<()> {
    log::info!("build index: {:?}", &cmd);
    for e in cmd.dest.into_iter() {
        let s: Vec<_> = e.split("=").collect();
        if s.len() != 2 {
            return Err(anyhow::anyhow!("invalid dest format"));
        }
        let (source, url) = (*s.get(0).unwrap(), *s.get(1).unwrap());
        let di = HiqSyncDest::try_from((source.to_string(), url.to_string()))
            .with_context(|| format!("failed to convert to HiqSyncDest, ({}, {})", source, url))?;

        let funcs = None;
        let (_, s) = hiq_data::store::get_store(&di, true, 0, &funcs, false)
            .await
            .with_context(|| format!("failed to get store"))?;

        s.build_index()
            .await
            .with_context(|| format!("failed to build index"))?;
    }

    Ok(())
}

async fn sync_cmd(cmd: SyncCommand) -> anyhow::Result<()> {
    log::info!("sync: {:?}", &cmd);
    let mut dest = Vec::new();
    for e in cmd.dest.into_iter() {
        let s: Vec<_> = e.split("=").collect();
        if s.len() != 2 {
            return Err(anyhow::anyhow!("invalid dest format"));
        }
        let (source, url) = (*s.get(0).unwrap(), *s.get(1).unwrap());
        let di = HiqSyncDest::try_from((source.to_string(), url.to_string()))
            .with_context(|| format!("failed to convert to HiqSyncDest, ({}, {})", source, url))?;
        dest.push(di);
    }
    let mut funcs = Vec::new();

    for e in cmd.funcs.into_iter() {
        let dt = HiqSyncDataType::try_from(e.as_str())
            .with_context(|| format!("failed to convert to HiqSyncDest, {}", e))?;
        funcs.push(dt)
    }
    let funcs = if funcs.len() > 0 { Some(funcs) } else { None };
    let (shutdown_tx, _) = broadcast::channel(1);
    let mut s = HiqSync::new(dest, shutdown_tx.subscribe(), funcs);
    tokio::select! {
        res = s.sync(cmd.skip_basic, cmd.concurrent, cmd.split_count) => {
            log::info!("sync done, result: {:?}", res);
        },
        _ = my_exit() => {
            log::info!("capture ctrl-c to exit");
            shutdown_tx.send(()).with_context(||"capture ctrl-c to exit error")?;
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

#[derive(FromArgs, PartialEq, Debug)]
/// HiqDataSync command.
struct HiqDataSync {
    /// 版本号
    #[argh(switch, short = 'v')]
    version: bool,

    /// 日志级别，默认info
    #[argh(option, short = 'l', default = "String::from(\"info\")")]
    level: String,

    #[argh(subcommand)]
    cmd: Option<HiqDataSubCommandEnum>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum HiqDataSubCommandEnum {
    Sync(SyncCommand),
    Build(BuildIndexCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// 数据同步命令
#[argh(subcommand, name = "sync")]
struct SyncCommand {
    /// 是否忽略同步基础数据，默认否
    #[argh(switch, short = 's')]
    skip_basic: bool,
    /// 并发获取数据任务数，默认为1
    #[argh(option, short = 'c', default = "4")]
    concurrent: usize,

    /// 股票切分份上（每份单独放一个task），默认为5
    #[argh(option, short = 'l', default = "5")]
    split_count: usize,
    /// 同步数据存储目的。“=”分割，前面一部分表示目标，后一部分表示url
    /// 如：file=/user/home/app, mongodb=mongodb://localhost:27017
    /// 支持的目标有: file, mongodb, mysql, clickhouse
    /// 可同时传递多个目标:
    /// 如：-d file=/user/home/app -d mongodb=mongodb://localhost:27017
    #[argh(option, short = 'd')]
    dest: Vec<String>,

    /// 同步的数据：
    /// trade_date, index_info, index_daily,
    /// stock_info, stock_daily, stock_index, stock_industry, stock_industry_detail,
    /// stock_industry_daily, stock_concept, stock_concept_detail,
    /// stock_concept_daily, stock_yjbb, stock_margin,
    /// fund_info, fund_net, fund_daily,
    /// bond_info, bond_daily,
    #[argh(option, short = 'f')]
    funcs: Vec<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// 重建索引
#[argh(subcommand, name = "build")]
struct BuildIndexCommand {
    /// 同步数据存储目的。“=”分割，前面一部分表示目标，后一部分表示url
    /// 如：file=/user/home/app, mongodb=mongodb://localhost:27017
    /// 支持的目标有: file, mongodb, mysql, clickhouse
    /// 可同时传递多个目标:
    /// 如：-d file=/user/home/app -d mongodb=mongodb://localhost:27017
    #[argh(option, short = 'd')]
    dest: Vec<String>,
}
