use std::sync::{Arc, RwLock};

use rwqdata::store::Loader;
use rwqtradecmm::{Account, Entrust, Event, Signal};

use crate::Result;

/// Context 将策略所使用到的功能集合一起供策略库使用。
pub struct Context {
    pub loader: Arc<Box<dyn Loader>>,
    pub account: Arc<Box<RwLock<Account>>>,
}

impl Context {
    pub async fn emit(&self, event: Event) -> Result<()> {
        todo!()
    }
    pub fn can_buy(&self, code: &str, price: f32, volume: usize) -> bool {
        todo!()
    }
    pub fn can_sell(&self, code: &str) -> usize {
        todo!()
    }
    pub fn can_cancel(&self, code: &str) -> Option<Entrust> {
        todo!()
    }
    pub async fn buy(&self, signal: Signal) -> Result<()> {
        todo!()
    }
    pub async fn sell(&self, signal: Signal) -> Result<()> {
        todo!()
    }
    pub async fn cancel(&self, signal: Signal) -> Result<()> {
        todo!()
    }

    pub async fn subscribe(&self, codes: Vec<String>) -> Result<()> {
        todo!()
    }
}
