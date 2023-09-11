mod fund_fetch;
mod trans_info;

pub use fund_fetch::*;

/// 返回默认的etf基金实现
pub fn fund_fetch() ->  FundFetch {
    FundFetch::new()
}
