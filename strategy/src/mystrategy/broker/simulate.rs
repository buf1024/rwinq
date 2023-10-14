use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use rwqtradecmm::{BrokerEvent, Entrust, EntrustStatus, Event};

use crate::{broker::Broker, context::Context, Result};

pub struct Simulate {
    pub entrust: RwLock<Vec<Entrust>>,
}

impl Simulate {
    pub fn new() -> Self {
        Self {
            entrust: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl Broker for Simulate {
    fn description(&self) -> String {
        String::from(
            r#"Simulate 模拟券商

通模拟券商，无条件成交。"#,
        )
    }
    fn name(&self) -> String {
        String::from("Simulate -- 模拟券商")
    }
    async fn on_entrust(&self, ctx: Arc<Context>, entrust: Entrust) -> Result<()> {
        let mut entrust = entrust.clone();
        entrust.broker_entrust_id = Some(entrust.id.to_string());
        entrust.time = Default::default();
        entrust.status = EntrustStatus::Commit;

        // 模拟委托提交事件
        ctx.emit(Event::Broker(BrokerEvent::Entrust(vec![entrust.clone()])))
            .await?;

        entrust.status = EntrustStatus::Deal;
        entrust.volume_deal = entrust.volume;
        {
            self.entrust.write().unwrap().push(entrust);
        }
        Ok(())
    }
    async fn on_poll(&self, ctx: Arc<Context>) -> Result<()> {
        let entrust = {
            let mut entrust = self.entrust.write().unwrap();
            if entrust.len() > 0 {
                Some(entrust.remove(0))
            } else {
                None
            }
        };
        if let Some(entrust) = entrust {
            // 模拟委托成交事件
            ctx.emit(Event::Broker(BrokerEvent::Entrust(vec![entrust.clone()])))
                .await?;
        }
        Ok(())
    }
}
