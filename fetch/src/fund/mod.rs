mod fund_fetch;
mod hi_fund_fetch;
mod hi_fund_info;

pub use fund_fetch::*;
pub use hi_fund_fetch::*;

/// 返回默认的etf基金实现
pub fn fund_fetch() -> impl FundFetch {
    HiqFundFetch::new()
}
