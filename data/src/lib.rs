//! 此模块有两个作用
//!
//! - 获取远程数据并存储到本地，支持的数据目的地为文件，MongoDB，MySQL  
//!   尽管数据可以同时同步到多个目的地，可对于终端用户而言，应该只使用一个。同步多个，数据量请求量和存储量成倍增加。
//!
//! - 提供简单统一接口访问各个数据源数据。
//!
//! 需要注意的是，这里的数据一般是不做修改的。如程序没bug存储到数据库中的数据都是最终状态。  

use thiserror::Error;

pub mod store;

pub mod sync;
pub mod syncer;
pub mod types;

/// 模块定义的错误码
#[derive(Error, Debug)]
pub enum Error {
    /// 接口未实现异常
    #[error("Function \"{0}\" not implement")]
    NotImpl(String),
    /// 获取远程数据错误
    #[error("Fetch error")]
    FetchError(#[from] rwqfetch::Error),
    /// 获取远程数据超时
    #[error("{0}")]
    FetchTimeout(String),
    /// shutdown错误
    #[error("{0}")]
    Shutdown(String),
    /// 自定义错误，为了偷懒，不定义太多错误。
    /// 未能上述表示的错误，一律用此表示
    #[error("{0}")]
    Custom(String),
}

/// 模块定义结果状态
pub type Result<T> = std::result::Result<T, Error>;

pub use sync::Sync;
pub use types::*;

pub use rwqfetch::*;
