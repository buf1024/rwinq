mod trans_info;

mod bond_fetch;

pub use bond_fetch::*;

/// 返回默认的可转债实现
pub fn bond_fetch() -> BondFetch {
    BondFetch::new()
}
