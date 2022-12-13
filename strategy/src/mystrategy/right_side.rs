use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bson::doc;
use hiq_data::store::Loader;

use crate::{
    stat_result, strategy_to_data_type, util::shadow, CommonParam, Error, Result, Strategy,
    StrategyResult, StrategyType,
};

#[derive(Debug, Clone)]
pub(crate) struct RightSide {
    cmm_params: CommonParam,
    min_rise_days: i32,
    max_shadow_pct: f32,
    min_volume_chg_pct: f32,
    min_amount_chg_pct: f32,
}

impl Default for RightSide {
    fn default() -> Self {
        Self {
            cmm_params: Default::default(),
            min_rise_days: 3,
            max_shadow_pct: 20.0,
            min_volume_chg_pct: -10.0,
            min_amount_chg_pct: -10.0,
        }
    }
}

#[async_trait]
impl Strategy for RightSide {
    fn name(&self) -> String {
        String::from("RightSide")
    }
    fn help(&self) -> String {
        return String::from(
            r###"名称: 右侧策略(基于日线)
                 说明: 选择右侧温和上涨的标的。
                      
                 参数: min_rise_days -- 最近最小连续上涨天数(默认: 3)
                       max_shadow_pct -- 上下影线最大百分比(默认: 20.0)
                       min_volume_chg_pct -- 最小成交量增加百分比(默认: -10.0)
                       min_amount_chg_pct -- 最小成交额增加百分比(默认: -10.0)"###,
        );
    }
    async fn prepare(
        &mut self,
        _loader: Arc<Box<dyn Loader>>,
        cmm_params: Option<CommonParam>,
        params: Option<HashMap<String, String>>,
    ) -> Result<()> {
        if let Some(cmm_params) = cmm_params {
            self.cmm_params = cmm_params.clone();
        }
        if let Some(params) = params {
            if params.contains_key("min_rise_days") {
                self.min_rise_days =
                    params.get("min_rise_days").unwrap().parse().map_err(|e| {
                        Error::Custom(format!("parse min_rise_days error: {:?}", e))
                    })?;
            }
            if params.contains_key("max_shadow_pct") {
                self.max_shadow_pct =
                    params.get("max_shadow_pct").unwrap().parse().map_err(|e| {
                        Error::Custom(format!("parse max_shadow_pct error: {:?}", e))
                    })?;
            }
            if params.contains_key("min_volume_chg_pct") {
                self.min_volume_chg_pct = params
                    .get("min_volume_chg_pct")
                    .unwrap()
                    .parse()
                    .map_err(|e| {
                        Error::Custom(format!("parse min_volume_chg_pct error: {:?}", e))
                    })?;
            }
            if params.contains_key("min_amount_chg_pct") {
                self.min_amount_chg_pct = params
                    .get("min_amount_chg_pct")
                    .unwrap()
                    .parse()
                    .map_err(|e| {
                        Error::Custom(format!("parse min_amount_chg_pct error: {:?}", e))
                    })?;
            }
        }
        Ok(())
    }
    async fn test(
        &self,
        loader: Arc<Box<dyn Loader>>,
        typ: StrategyType,
        code: String,
        name: String,
    ) -> Result<Option<StrategyResult>> {
        log::debug!("testing typ: {:?}, code: {}, name: {}", &typ, code, name);
        let test_end_date = self.cmm_params.test_end_date.unwrap();
        let test_trade_days = self.cmm_params.test_trade_days.unwrap();

        let dt_str = loader.naive_date_time_to_datetime_str(&test_end_date).map_err(|e| {
            Error::Custom(format!(
                "naive_date_to_datetime_str error: {}",
                e.to_string()
            ))
        })?;
        let kdata = loader
            .load_daily(
                strategy_to_data_type(typ),
                doc! {"code": &code, "trade_date": {"$lte": dt_str}},
                doc! {"trade_date": -1},
                Some(test_trade_days),
            )
            .await
            .map_err(|e| Error::Custom(format!("load_daily error: {}", e.to_string())))?;

        if kdata.len() < test_trade_days as usize {
            return Ok(None);
        }

        let mut hit_days = 0;
        let (mut hit, mut hit_max) = (0, 0);
        for (index, data) in kdata.iter().enumerate() {
            let (chg_pct, volume_chg_pct, amount_chg_pct) =
                (data.chg_pct, data.volume_chg_pct, data.amount_chg_pct);
            let (open, close, high, low) = (data.open, data.close, data.high, data.low);
            let (_, u_shadow, _, l_shadow) =
                shadow(close * (1.0 + chg_pct / 100.0), open, close, low, high);
            if chg_pct > 0.0
                && volume_chg_pct >= self.min_volume_chg_pct
                && amount_chg_pct >= self.min_amount_chg_pct
                && u_shadow <= self.max_shadow_pct
                && l_shadow <= self.max_shadow_pct
            {
                hit_days = hit_days + 1;
                if hit_days == self.min_rise_days {
                    hit = index;
                }
                hit_max = index;
                continue;
            }
            break;
        }

        if hit_days < self.min_rise_days {
            return Ok(None);
        }

        let stat = stat_result(&kdata, hit, hit_max)?;
        let mut mark = HashMap::new();
        let hit_bar = kdata.get(hit).unwrap();
        let hit_mark = serde_json::to_string(hit_bar).map_err(|e| {
            Error::Custom(format!(
                "hit_bar serde_json::to_string error: {}",
                e.to_string()
            ))
        })?;
        mark.insert(hit_bar.trade_date.date(), hit_mark);

        let hit_max_bar = kdata.get(hit_max).unwrap();
        let hit_max_mark = serde_json::to_string(hit_max_bar).map_err(|e| {
            Error::Custom(format!(
                "hit_max_bar serde_json::to_string error: {}",
                e.to_string()
            ))
        })?;
        mark.insert(hit_max_bar.trade_date.date(), hit_max_mark);

        Ok(Some(StrategyResult::new(
            code.clone(),
            name.clone(),
            Some(mark),
            Some(stat),
        )))
    }
}
