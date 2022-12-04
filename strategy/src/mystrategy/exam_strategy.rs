use std::sync::Arc;

use async_trait::async_trait;
use hiq_data::store::Loader;

use crate::{Result, Strategy, StrategyResult, StrategyType};

#[derive(Debug, Clone)]
pub(crate) struct ExamStrategy {}
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
