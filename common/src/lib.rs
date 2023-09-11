//! A股最基本数据结构，包括可转债，etf基金已经股票。
//!
//! 此公共模块所定义基本数据机构目的是为了给其他模块共享使用

pub mod cmm;
pub use cmm::*;

pub mod bond;
pub use bond::*;

pub mod fund;
pub use fund::*;

pub mod stock;
pub use stock::*;

pub fn naive_dt_serialize<S>(ndt: &chrono::NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let dt = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(*ndt, chrono::Utc);

    bson::serde_helpers::serialize_chrono_datetime_as_bson_datetime(&dt, s)
}

pub fn naive_dt_deserialize<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let dt = bson::serde_helpers::deserialize_chrono_datetime_from_bson_datetime(deserializer)?;
    Ok(dt.naive_local())
}
