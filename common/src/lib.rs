//! A股最基本数据结构，包括可转债，etf基金已经股票。
//!
//! 此公共模块所定义基本数据机构目的是为了给其他模块共享使用

pub mod cmm;
use chrono::{LocalResult, TimeZone, Utc};
pub use cmm::*;

pub mod bond;
pub use bond::*;

pub mod fund;
pub use fund::*;

pub mod stock;
use serde::{Deserialize, Serializer};
pub use stock::*;

/// 股票市场： 深圳/上海/上海
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[repr(u8)]
pub enum Market {
    SZ = 0,
    SH = 1,
    BJ = 2,
}

impl From<i32> for Market {
    fn from(v: i32) -> Self {
        match v {
            0 => Market::SZ,
            1 => Market::SH,
            2 => Market::BJ,
            _ => Market::SH,
        }
    }
}

/// 市场交易类型： 可转债，etf基金，股票
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Deserialize, serde::Serialize,
)]
#[repr(u8)]
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

pub fn naive_dt_serialize<S>(ndt: &chrono::NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // let dt = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(*ndt, chrono::Utc);

    // bson::serde_helpers::serialize_chrono_datetime_as_bson_datetime(&dt, s)
    // s.serialize_i64(dt.timestamp())
    // ndt.timestamp()
    // ndt.serialize(s)
    s.serialize_i64(ndt.timestamp())
}

pub fn naive_dt_deserialize<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let ts = i64::deserialize(deserializer)?;
    let dt = match Utc.timestamp_opt(ts, 0) {
        LocalResult::Single(dt) => dt,
        _ => {
            if ts < 0 {
                chrono::DateTime::<Utc>::MIN_UTC
            } else {
                chrono::DateTime::<Utc>::MAX_UTC
            }
        }
    };
    Ok(dt.naive_local())
}
