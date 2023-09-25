use rwqcmm::Quot;
use serde::{Deserialize, Serialize};

use crate::{TradeTime, Uuid};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Position {
    #[serde(
        serialize_with = "crate::uuid_serialize",
        deserialize_with = "crate::uuid_deserialize"
    )]
    pub id: Uuid,
    pub name: String, // 股票名称
    pub code: String, // 股票代码
    #[serde(
        serialize_with = "crate::trade_time_serialize",
        deserialize_with = "crate::trade_time_deserialize"
    )]
    pub time: TradeTime, // 首次建仓时间

    pub volume: u32,           // 持仓量
    pub volume_available: u32, // 可用持仓量
    pub volume_frozen: u32,    // 可用持仓量

    pub fee: f32,   // 持仓费用
    pub price: f32, // 平均持仓价

    pub now: f32,       // 最新价
    pub max_price: f32, // 最高价
    pub min_price: f32, // 最低价

    pub profit_rate: f32,     // 盈利比例
    pub max_profit_rate: f32, // 最大盈利比例
    pub min_profit_rate: f32, // 最小盈利比例

    pub profit: f32,     // 盈利
    pub max_profit: f32, // 最大盈利
    pub min_profit: f32, // 最小盈利

    #[serde(
        serialize_with = "crate::opt_trade_time_serialize",
        deserialize_with = "crate::opt_trade_time_deserialize"
    )]
    pub max_profit_time: Option<TradeTime>, // 最大盈利时间
    #[serde(default)]
    #[serde(
        serialize_with = "crate::opt_trade_time_serialize",
        deserialize_with = "crate::opt_trade_time_deserialize"
    )]
    pub min_profit_time: Option<TradeTime>, // 最小盈利时间
}

impl Position {
    pub fn on_quot(&mut self, quot: &Quot) {
        self.now = quot.now;
        if self.max_price < self.now {
            self.max_price = self.now;
        }
        if self.min_price > self.now {
            self.min_price = self.now
        }
        self.profit = (self.now - self.price) * self.volume as f32 - self.fee;
        self.profit_rate = self.profit / (self.price * self.volume as f32 + self.fee);
        if self.profit > self.max_profit {
            self.max_profit = self.profit;
            self.max_profit_time = Some(TradeTime(quot.time.clone()));
            self.max_profit_rate = self.profit_rate;
        }

        if self.profit < self.min_profit {
            self.min_profit = self.profit;
            self.min_profit_time = Some(TradeTime(quot.time.clone()));
            self.min_profit_rate = self.profit_rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Position;

    #[test]
    fn test_ser() {
        let mut pos: Position = Default::default();
        pos.max_profit_time = Some(Default::default());

        let js = serde_json::to_string(&pos).unwrap();
        println!("js={}", js)
    }
}
