pub mod quotation;
pub use quotation::*;

pub mod investor;
pub use investor::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Function \"{0}\" not implement")]
    NotImpl(&'static str),
    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;
