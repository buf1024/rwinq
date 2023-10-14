use std::sync::RwLock;

use rwqdata::store::Loader;
use rwqtradecmm::{Account, Entrust, Event, Signal, TradeType};
use tokio::sync::mpsc;

use crate::Result;

/// Context 将策略所使用到的功能集合一起供策略库使用。
pub struct Context {
    pub loader: Box<dyn Loader>,
    pub account: Box<RwLock<Account>>,
    pub event_tx: mpsc::Sender<Event>,
}

impl Context {
    pub async fn emit(&self, event: Event) -> Result<()> {
        match event {
            Event::Signal(_) => todo!(),
            Event::Subscribe(_) => todo!(),
            Event::Entrust(_) => todo!(),
            Event::Broker(_) => todo!(),
        }

        Ok(())
    }
    pub fn can_buy(&self, price: f32, volume: u32) -> bool {
        let account = self.account.read().unwrap();
        account.cash_available > account.get_est_cost(TradeType::Buy, price, volume)
    }
    pub fn can_sell(&self, code: &str) -> u32 {
        let account = self.account.read().unwrap();
        let (_, volume) = account.get_position_volume(code);
        volume
    }
    pub fn can_cancel(&self, code: &str) -> Vec<Entrust> {
        let account = self.account.read().unwrap();
        account.get_active_entrust(code)
    }
    pub async fn buy(&self, signal: Signal) -> Result<()> {
        self.emit(Event::Signal(signal)).await
    }
    pub async fn sell(&self, signal: Signal) -> Result<()> {
        self.emit(Event::Signal(signal)).await
    }
    pub async fn cancel(&self, signal: Signal) -> Result<()> {
        self.emit(Event::Signal(signal)).await
    }

    /// 订阅行情
    pub async fn subscribe(&self, codes: Vec<String>) -> Result<()> {
        self.emit(Event::Subscribe(codes)).await
    }
}
