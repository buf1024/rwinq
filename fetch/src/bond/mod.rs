mod hiq_bond_info;

mod bond_fetch;
mod hiq_bond_fetch;

pub use bond_fetch::*;
pub use hiq_bond_fetch::*;

pub fn bond_fetch() -> impl BondFetch {
    HiqBondFetch::new()
}
