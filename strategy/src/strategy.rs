#![allow(unused_variables)]

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDate;
use hiq_data::store::Loader;
use serde::{Serialize, Deserialize};

use crate::{Result, StrategyType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Marker {
    pub key: String,
    pub val: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stat {
    pub chg_pct: [f32; 5],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategyResult {
    pub code: String,
    pub name: String,
    pub marker: Option<Vec<Marker>>,
    pub stat: Option<Stat>,
}

#[derive(Clone, Debug)]
pub struct CommonParam {
    pub test_end_date: Option<NaiveDate>,
    pub min_trade_days: Option<i64>,
}

impl Default for CommonParam {
    fn default() -> Self {
        Self {
            test_end_date: Default::default(),
            min_trade_days: Some(60),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_strategy() {}
}
