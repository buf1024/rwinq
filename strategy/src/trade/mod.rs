use async_trait::async_trait;

use crate::{context::Context, Params, Result};

#[async_trait]
pub trait Strategy: Sync + Send {
    fn help(&self) -> String {
        String::from("")
    }
    fn name(&self) -> String {
        String::from("")
    }
    async fn prepare(&mut self, ctx: Context, params: Option<Params>) -> Result<()> {
        Ok(())
    }
    async fn on_open(&self) -> Result<()> {
        Ok(())
    }
    async fn on_close(&self) -> Result<()> {
        Ok(())
    }
    async fn on_quot(&self) -> Result<()> {
        Ok(())
    }
}

// emit(buy, adf, 100)
