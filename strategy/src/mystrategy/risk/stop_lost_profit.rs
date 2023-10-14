use std::sync::Arc;

use async_trait::async_trait;
use rwqdata::RtQuot;
use rwqtradecmm::{Position, Signal, SignalSource, TradeType};

use crate::{context::Context, risk::Risk, Result};

pub struct StopLostProfit {
    pub profit: Option<f32>,
    pub profit_rate: Option<f32>,
    pub lost: Option<f32>,
    pub lost_rate: Option<f32>,
}

impl StopLostProfit {
    pub fn new(
        profit: Option<f32>,
        profit_rate: Option<f32>,
        lost: Option<f32>,
        lost_rate: Option<f32>,
    ) -> Self {
        Self {
            profit,
            profit_rate,
            lost,
            lost_rate,
        }
    }
    fn get_price(&self, quots: &RtQuot, code: &str) -> f32 {
        quots
            .get(code)
            .and_then(|quot| {
                // 买3快速止损
                let (_, mut price) = quot.bid.3;
                if price <= 0.0001 {
                    price = quot.now
                }
                Some(price)
            })
            .unwrap_or_default()
    }
    fn get_signal(&mut self, position: &Position, price: f32, desc: String) -> Signal {
        Signal {
            id: Default::default(),
            typ: TradeType::Sell,
            source: SignalSource::Risk(String::from("StopLostProfit")),
            code: position.code.clone(),
            name: position.name.clone(),
            time: Default::default(),
            price,
            volume: position.volume_available,
            desc,
        }
    }
}

impl Default for StopLostProfit {
    fn default() -> Self {
        Self {
            profit: Default::default(),
            profit_rate: Some(0.2),
            lost: Default::default(),
            lost_rate: Some(0.03),
        }
    }
}

#[async_trait]
impl Risk for StopLostProfit {
    fn description(&self) -> String {
        String::from(
            r#"StopLostProfit 止损止盈

通过`profit`/`profit_rate`设置止盈，默认20%止盈。

通过`lost`/`lost_rate`设置止损，默认3%止损。"#,
        )
    }
    fn name(&self) -> String {
        String::from("StopLostProfit -- 止损止盈")
    }

    async fn on_risk(&mut self, ctx: Arc<Context>, quots: RtQuot) -> Result<()> {
        let positions = { ctx.account.read().unwrap().position.clone() };
        for (_, position) in positions.iter() {
            let mut signal = None;
            if let Some(profit) = self.profit {
                if position.profit >= profit {
                    signal = Some(self.get_signal(
                        position,
                        self.get_price(&quots, &position.code),
                        format!(
                            "StopLostProfit快速止盈({}) profit={}",
                            profit, position.profit
                        ),
                    ));
                }
            }
            if signal.is_none() {
                if let Some(lost) = self.lost {
                    if position.profit <= lost {
                        signal = Some(self.get_signal(
                            position,
                            self.get_price(&quots, &position.code),
                            format!(
                                "StopLostProfit快速止损({}) profit_rate={}",
                                lost, position.profit
                            ),
                        ));
                    }
                }
            }
            if signal.is_none() {
                if let Some(lost_rate) = self.lost_rate {
                    if position.profit_rate <= lost_rate {
                        signal = Some(self.get_signal(
                            position,
                            self.get_price(&quots, &position.code),
                            format!(
                                "StopLostProfit快速止损({}%) profit_rate={}%",
                                lost_rate * 100.0,
                                position.profit_rate * 100.0
                            ),
                        ));
                    }
                }
            }
            if let Some(signal) = signal {
                ctx.sell(signal).await?;
            }
        }
        Ok(())
    }
}
