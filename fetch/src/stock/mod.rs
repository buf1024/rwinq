mod hiq_stock_fetch;
mod hiq_stock_info;
mod stock_fetch;

pub use hiq_stock_fetch::*;
pub use stock_fetch::*;

pub fn stock_fetch() -> impl StockFetch {
    HiqStockFetch::new()
}
