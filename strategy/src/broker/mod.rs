#![allow(unused)]
use std::sync::Arc;

use async_trait::async_trait;
use rwqtradecmm::{Entrust, Event, QuotEvent};

use crate::{context::Context, Params, Result};

/// 券商接口
///
///  接收事件 买委托事件, 卖委托事件, 撤销委托事件
///
/// buy(买), sell(卖), cancel(撤销)委托成功或失败均产生委托结果事件
/// buy(买), sell(卖), cancel(撤销)成交或撤销均产生事件
///
// 券商产生的事件: 委托提交事件, 委托(买,卖)成交事件, 撤销事件, 资金同步事件, 持仓同步事件
#[async_trait]
pub trait Broker: Sync + Send {
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
    async fn on_entrust(&self, ctx: Arc<Context>, entrust: Entrust) -> Result<()> {
        Ok(())
    }
    async fn on_poll(&self, ctx: Arc<Context>) -> Result<()> {
        Ok(())
    }
}

// emit(buy, adf, 100)
