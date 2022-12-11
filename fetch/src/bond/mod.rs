mod hiq_bond_info;

mod bond_fetch;
mod hiq_bond_fetch;

pub use bond_fetch::*;
pub use hiq_bond_fetch::*;

/// 返回默认的可转债实现
pub fn bond_fetch() -> impl BondFetch {
    HiqBondFetch::new()
}
