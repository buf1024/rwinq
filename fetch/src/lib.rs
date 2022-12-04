use once_cell::sync::Lazy;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, CONNECTION, PRAGMA, USER_AGENT,
};
use thiserror::Error;

mod bond;
pub use bond::*;

mod stock;
pub use stock::*;

mod fund;
pub use fund::*;

mod comm;
pub use comm::fetch_trade_date;

mod util;

pub use hiq_common::*;

pub use util::to_std_code;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Remote http request error")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse json error")]
    JsonError(#[from] serde_json::Error),
    #[error("Function \"{0}\" not implement")]
    NotImpl(&'static str),
    #[error("{0}")]
    Custom(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) static HTTP_CMM_HEADER: Lazy<HeaderMap> = Lazy::new(|| {
    let mut header = HeaderMap::new();
    header.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    header.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    header.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    header.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64) \
                      AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36",
        ),
    );
    header.insert(ACCEPT, HeaderValue::from_static("*/*"));
    header.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    header
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Market {
    SZ = 0,
    SH = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketType {
    Bond = 0,
    Fund = 1,
    Stock = 2,
}

impl From<i32> for MarketType {
    fn from(v: i32) -> Self {
        match v {
            0 => MarketType::Bond,
            1 => MarketType::Fund,
            2 => MarketType::Stock,
            _ => MarketType::Stock
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustFactor {
    NFQ = 0,
    QFQ = 1,
    HFQ = 2,
}
