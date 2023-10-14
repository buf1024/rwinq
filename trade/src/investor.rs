use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use rwqdata::store::Loader;
use rwqstrategy::{broker::Broker, risk::Risk, trade::Strategy};
use rwqtradecmm::Account;

use crate::Result;

pub struct Investor {
    pub broker: Box<dyn Broker>,
    pub strategy: Box<dyn Strategy>,
    pub risk: Box<dyn Risk>,

    pub account: Arc<Box<RwLock<Account>>>,
    pub loader: Arc<Box<dyn Loader>>,
}

impl Investor {
    pub fn new(
        broker: Box<dyn Broker>,
        strategy: Box<dyn Strategy>,
        risk: Box<dyn Risk>,
        account: Account,
        loader: Box<dyn Loader>,
    ) -> Self {
        Self {
            broker,
            strategy,
            risk,
            account: Arc::new(Box::new(RwLock::new(account))),
            loader: Arc::new(loader),
        }
    }
    pub async fn invest(&mut self) -> Result<()> {
        Ok(())
    }
    pub async fn backtest(&mut self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub trait InvestVisitor {
    async fn visit(&self, investor: &Investor);
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_investor() {
        
    }
}