use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// 交易类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TradeType {
    Buy = 0,
    Sell = 1,
    Cancel = 2,
}

impl Display for TradeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TradeType::Sell => "卖出",
            TradeType::Buy => "买入",
            TradeType::Cancel => "撤销",
        };
        write!(f, "{}", s)
    }
}

impl Default for TradeType {
    fn default() -> Self {
        TradeType::Cancel
    }
}

pub fn uuid() -> String {
    uuid::Uuid::new_v4().as_simple().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uuid(pub String);
impl Default for Uuid {
    fn default() -> Self {
        Self(uuid())
    }
}
impl Deref for Uuid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn uuid_serialize<S>(uid: &Uuid, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    uid.0.serialize(s)
}

pub fn uuid_deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let uid = String::deserialize(deserializer)?;
    Ok(Uuid(uid))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeTime(pub NaiveDateTime);
impl Default for TradeTime {
    fn default() -> Self {
        Self(Local::now().naive_local())
    }
}
impl Deref for TradeTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TradeTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn trade_time_serialize<S>(dt: &TradeTime, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    rwqcmm::naive_dt_serialize(&dt.0, s)
}

pub fn trade_time_deserialize<'de, D>(deserializer: D) -> Result<TradeTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let dt = rwqcmm::naive_dt_deserialize(deserializer)?;
    Ok(TradeTime(dt))
}

pub fn opt_trade_time_serialize<S>(dt: &Option<TradeTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(t) => rwqcmm::naive_dt_serialize(&t.0, s),
        None => None::<Option<TradeTime>>.serialize(s),
    }
}

pub fn opt_trade_time_deserialize<'de, D>(deserializer: D) -> Result<Option<TradeTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(rwqcmm::naive_dt_deserialize(deserializer)
        .map(|dt| TradeTime(dt))
        .ok())
}
