#![allow(unused)]
use std::sync::Arc;

use async_trait::async_trait;
use rwqdata::RtQuot;
use rwqtradecmm::QuotEvent;

use crate::{context::Context, Params, Result};

#[async_trait]
pub trait Risk: Sync + Send {
    /// 策略说明，使用的是md格式
    fn description(&self) -> String {
        String::from("")
    }
    fn name(&self) -> String {
        String::from("Risk")
    }
    async fn init(&mut self, ctx: Arc<Context>, params: Option<Params>) -> Result<()> {
        Ok(())
    }
    async fn destroy(&mut self, ctx: Arc<Context>) -> Result<()> {
        Ok(())
    }
    async fn on_start(&mut self, ctx: Arc<Context>) -> Result<()> {
        Ok(())
    }
    async fn on_open(&mut self, ctx: Arc<Context>, event: QuotEvent) -> Result<()> {
        Ok(())
    }
    async fn on_close(&mut self, ctx: Arc<Context>, event: QuotEvent) -> Result<()> {
        Ok(())
    }
    async fn on_end(&mut self, ctx: Arc<Context>) -> Result<()> {
        Ok(())
    }
    async fn on_risk(&mut self, ctx: Arc<Context>, quots: RtQuot) -> Result<()> {
        Ok(())
    }
}

// emit(buy, adf, 100)
