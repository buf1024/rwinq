//! 网上获取数据，数据类型包括可转债，ETF基金，股票。  
//! 这里获取的数据是最基本的数据，不排除以后会新增其他类型的数据。  
//! 数据的来源不一定是固定一个地方。  
//! 需要注意的是，获取数据时，如果并发获取，需要要限制并发数量，否则可能会被封ip  
use once_cell::sync::Lazy;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, CONNECTION, PRAGMA, USER_AGENT,
};
use thiserror::Error;

pub mod bond;
pub use bond::*;

pub mod stock;
pub use stock::*;

pub mod fund;
pub use fund::*;

pub mod comm;
pub use comm::*;

pub mod util;

pub use rwqcmm::*;

pub use util::to_std_code;

/// 模块定义的错误码
#[derive(Error, Debug)]
pub enum Error {
    /// 远程请求数据异常
    #[error("Remote request error")]
    RequestError(#[from] reqwest::Error),
    /// 解释json数据异常
    #[error("Parse json error")]
    JsonError(#[from] serde_json::Error),
    /// 接口未实现异常
    #[error("Function \"{0}\" not implement")]
    NotImpl(String),
    /// 自定义错误，为了偷懒，不定义太多错误。
    /// 未能上述表示的错误，一律用此表示
    #[error("{0}")]
    Custom(String),
}

/// 模块定义结果状态
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

/// 股票市场： 深圳或上海
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Market {
    SZ = 0,
    SH = 1,
}

/// 市场交易类型： 可转债，etf基金，股票
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MarketType {
    Bond = 0,
    Fund = 1,
    #[default]
    Stock = 2,
}

impl From<i32> for MarketType {
    fn from(v: i32) -> Self {
        match v {
            0 => MarketType::Bond,
            1 => MarketType::Fund,
            2 => MarketType::Stock,
            _ => MarketType::Stock,
        }
    }
}

/// 复权方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdjustFactor {
    NFQ = 0,
    QFQ = 1,
    #[default]
    HFQ = 2,
}
