use thiserror::Error;

pub mod store;

pub mod sync;
pub mod syncer;
pub mod types;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Function \"{0}\" not implement")]
    NotImpl(&'static str),
    #[error("{0}")]
    Custom(&'static str),
    #[error("Fetch error")]
    FetchError(#[from] hiq_fetch::Error),
    #[error("{0}")]
    FetchTimeout(&'static str),
    #[error("{0}")]
    Shutdown(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

pub use sync::HiqSync;
pub use types::*;

pub use hiq_fetch::*;
