use crate::Result;
use async_trait::async_trait;
use rwqtradecmm::QuotEvent;

#[async_trait]
pub trait Quotation {
    async fn subscribe(&mut self, codes: Vec<String>) -> Result<()>;
    async fn get_quot(&mut self, codes: Option<Vec<String>>) -> Result<QuotEvent>;
}

pub fn backtest() -> Box<dyn Quotation> {
    Box::new(BacktestQuotation::new())
}

pub fn realtime() -> Box<dyn Quotation> {
    Box::new(RealtimeQuotation::new())
}


struct BacktestQuotation {}

impl BacktestQuotation {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Quotation for BacktestQuotation {
    async fn subscribe(&mut self, codes: Vec<String>) -> Result<()> {
        Ok(())
    }
    async fn get_quot(&mut self, codes: Option<Vec<String>>) -> Result<QuotEvent> {
        todo!()
    }
}

struct RealtimeQuotation {
    codes: Vec<String>,
}

impl RealtimeQuotation {
    fn new() -> Self {
        Self { codes: vec![] }
    }
}

#[async_trait]
impl Quotation for RealtimeQuotation {
    async fn subscribe(&mut self, codes: Vec<String>) -> Result<()> {
        let codes: Vec<_> = codes
            .into_iter()
            .filter(|code| !self.codes.contains(code))
            .collect();
        if codes.len() > 0 {
            self.codes.extend(codes.into_iter());
        }
        Ok(())
    }
    async fn get_quot(&mut self, codes: Option<Vec<String>>) -> Result<QuotEvent> {
        todo!()
    }
}
