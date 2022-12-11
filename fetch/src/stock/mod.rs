mod hiq_stock_fetch;
mod hiq_stock_info;
mod stock_fetch;

pub use hiq_stock_fetch::*;
pub use stock_fetch::*;

/// 返回默认的股票实现
pub fn stock_fetch() -> impl StockFetch {
    HiqStockFetch::new()
}
