use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bson::doc;
use rwqdata::store::Loader;

use crate::{
    stat_result, strategy_to_data_type, Error, Result, Strategy, StrategyResult, StrategyType,
};

#[derive(Debug, Clone)]
pub(crate) struct ExamStrategy {}
impl Default for ExamStrategy {
    fn default() -> Self {
        Self {}
    }
}
#[async_trait]
impl Strategy for ExamStrategy {
    fn name(&self) -> String {
        String::from("ExamStrategy")
    }
    fn help(&self) -> String {
        String::from("实例策略")
    }
    async fn test(
        &self,
        loader: Arc<Box<dyn Loader>>,
        typ: StrategyType,
        code: String,
        name: String,
    ) -> Result<Option<StrategyResult>> {
        let codes = vec![
            "sz002805".to_string(),
            "sz300827".to_string(),
            "sz000762".to_string(),
        ];
        if codes.contains(&code) {
            let data = loader
                .load_daily(
                    strategy_to_data_type(typ),
                    doc! {"code": &code},
                    doc! {"trade_date": -1},
                    Some(60),
                )
                .await
                .map_err(|e| Error::Custom(format!("load_daily error: {}", e.to_string())))?;
            let stat = stat_result(&data, 3, 15)
                .map_err(|e| Error::Custom(format!("stat result error: {}", e.to_string())))?;

            let mut mark = HashMap::new();
            let data0 = data.get(0).unwrap();
            let data1 = data.get(1).unwrap();
            mark.insert(
                data0.trade_date.date(),
                format!("data0 marker: {:?}", &data0.trade_date),
            );
            mark.insert(
                data1.trade_date.date(),
                format!("data1 marker: {:?}", &data1.trade_date),
            );

            let rs = StrategyResult {
                code,
                name,
                mark: Some(mark),
                stat: Some(stat),
            };
            return Ok(Some(rs));
        }

        Ok(None)
    }
}
