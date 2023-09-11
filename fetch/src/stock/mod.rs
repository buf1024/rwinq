mod stock_fetch;
mod trans_info;

pub use stock_fetch::*;

/// 返回默认的股票实现
pub fn stock_fetch() -> StockFetch {
    StockFetch::new()
}
