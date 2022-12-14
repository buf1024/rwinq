#![allow(unused_variables)]

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use hiq_data::{store::Loader, Bar};
use serde::{Deserialize, Serialize};

use crate::{Result, StrategyType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stat {
    pub hit_chg_pct: [f32; 5], // 1, 2, 4, 8, now 涨幅
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub low: NaiveDate,
    pub high: NaiveDate,
    pub hit: NaiveDate,
    pub hit_max: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategyResult {
    pub code: String,
    pub name: String,
    pub mark: Option<HashMap<NaiveDate, String>>,
    pub stat: Option<Stat>,
}

impl StrategyResult {
    pub fn new(
        code: String,
        name: String,
        mark: Option<HashMap<NaiveDate, String>>,
        stat: Option<Stat>,
    ) -> Self {
        Self {
            code,
            name,
            mark,
            stat,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommonParam {
    pub test_end_date: Option<NaiveDateTime>,
    pub test_trade_days: Option<i64>,
}

impl CommonParam {
    pub fn new(test_end_date: Option<NaiveDateTime>, test_trade_days: Option<i64>) -> Self {
        Self {
            test_end_date,
            test_trade_days,
        }
    }
}

impl Default for CommonParam {
    fn default() -> Self {
        let n = Local::now().date_naive();
        let n = NaiveDateTime::new(n, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        Self {
            test_end_date: Some(n),
            test_trade_days: Some(60)
        }
    }
}

#[async_trait]
pub trait Strategy: Sync + Send {
    fn help(&self) -> String {
        String::from("")
    }
    fn name(&self) -> String {
        String::from("")
    }
    async fn prepare(
        &mut self,
        loader: Arc<Box<dyn Loader>>,
        cmm_params: Option<CommonParam>,
        params: Option<HashMap<String, String>>,
    ) -> Result<()> {
        Ok(())
    }
    fn accept(&self) -> Vec<StrategyType> {
        vec![StrategyType::Stock]
    }
    async fn test(
        &self,
        loader: Arc<Box<dyn Loader>>,
        typ: StrategyType,
        code: String,
        name: String,
    ) -> Result<Option<StrategyResult>> {
        Ok(None)
    }
}

pub fn stat_result(data: &Vec<Bar>, hit: usize, hit_max: usize) -> Result<Stat> {
    if hit >= data.len() || hit_max > data.len() || hit_max < hit {
        return Err(crate::Error::Custom(format!(
            "hit={} or hit_max={}, out of range(0~{}) or invalid",
            hit,
            hit_max,
            data.len()
        )));
    }
    let hit_bar = data
        .get(hit)
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;
    let hit_max_bar = data
        .get(hit_max)
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;
    let start_bar = data
        .last()
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;
    let end_bar = data
        .first()
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;

    let (mut low, mut high) = (0.0, 0.0);
    let (mut low_index, mut high_index) = (0usize, 0usize);
    for (i, val) in data.iter().enumerate() {
        if val.close < low {
            low = val.close;
            low_index = i;
        }
        if val.close > high {
            high = val.close;
            high_index = i;
        }
    }
    let low_bar = data
        .get(low_index)
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;
    let high_bar = data
        .get(high_index)
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;

    let mut hit_chg_pct: [f32; 5] = [0.0; 5];
    let mut hit_chg_pct_index = 0;
    for i in 0..hit {
        let index = hit - i - 1;
        let step = i + 1;
        if step == 1 || step == 2 || step == 4 || step == 8 {
            let bar = data
                .get(index)
                .ok_or(crate::Error::Custom("invalid index".to_string()))?;
            hit_chg_pct[hit_chg_pct_index] = (bar.close - hit_bar.close) * 100.0 / hit_bar.close;
            hit_chg_pct_index += 1;
        }
    }
    let now_bar = data
        .get(0)
        .ok_or(crate::Error::Custom("invalid index".to_string()))?;
    hit_chg_pct[4] = (now_bar.close - hit_bar.close) * 100.0 / hit_bar.close;
    let stat = Stat {
        hit_chg_pct, // 1, 2, 4, 8, now 涨幅
        start: start_bar.trade_date.date(),
        end: end_bar.trade_date.date(),
        low: low_bar.trade_date.date(),
        high: high_bar.trade_date.date(),
        hit: hit_bar.trade_date.date(),
        hit_max: hit_max_bar.trade_date.date(),
    };
    Ok(stat)
}

pub type ProgressFunc = Box<dyn Fn(&str, &str, usize, usize, f32) -> () + Sync + Send>;
