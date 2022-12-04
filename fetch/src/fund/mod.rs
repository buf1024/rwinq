mod fund_fetch;
mod hi_fund_fetch;
mod hi_fund_info;

pub use fund_fetch::*;
pub use hi_fund_fetch::*;

pub fn fund_fetch() -> impl FundFetch {
    HiqFundFetch::new()
}
