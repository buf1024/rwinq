use std::sync::Arc;

use async_trait::async_trait;
use hiq_strategy::{Result, StrategyType, store::Loader, StrategyResult, Strategy};


#[derive(Debug, Clone)]
pub(crate) struct TestStrategy {}
#[async_trait]
impl Strategy for TestStrategy {
    fn name(&self) -> String {
        String::from("TestStrategy")
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
