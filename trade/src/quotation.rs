use std::ops::{Deref, DerefMut};

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::{Local, NaiveDate, NaiveDateTime};
use rwqdata::{fetch_is_trade_date, fetch_rt_quot, RtQuot};
use rwqtradecmm::{QuotEvent, QuotOpts};

#[async_trait]
pub trait Quotation {
    async fn subscribe(&mut self, codes: &Vec<String>);
    async fn fetch(&mut self, codes: &Option<Vec<String>>) -> Result<Option<QuotEvent>>;
}

pub fn backtest(opts: QuotOpts) -> Box<dyn Quotation> {
    Box::new(BacktestQuotation::new())
}

pub fn realtime(opts: QuotOpts) -> Box<dyn Quotation> {
    Box::new(RealtimeQuotation::new(opts))
}

struct MyQuotation {
    opts: QuotOpts,
    codes: Vec<String>,

    is_start: bool,
    is_end: bool,

    trade_date: Option<NaiveDate>,
    base_event: [bool; 4],
}

impl MyQuotation {
    fn is_trading(&self) -> bool {
        self.trade_date.is_some()
            && ((self.base_event[0] && !self.base_event[1])
                || (self.base_event[2] && !self.base_event[3]))
    }

    fn fire_base_event(&mut self, idx: usize) -> Result<Option<QuotEvent>> {
        for i in 0..=idx {
            if !self.base_event[i] {
                self.base_event[i] = true;
                return match i {
                    0 => Ok(Some(QuotEvent::MorningOpen)),
                    1 => Ok(Some(QuotEvent::MorningClose)),
                    2 => Ok(Some(QuotEvent::NoonOpen)),
                    3 => Ok(Some(QuotEvent::NoonClose)),
                    _ => Ok(None),
                };
            }
        }
        Ok(None)
    }
    fn get_base_event(&mut self, n: &NaiveDateTime) -> Result<Option<QuotEvent>> {
        if let Some(trade_date) = &self.trade_date {
            let date = trade_date.format("%Y-%m-%d").to_string();
            let m_start = NaiveDateTime::parse_from_str(
                format!("{} 09:15:00", date).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();
            let m_end = NaiveDateTime::parse_from_str(
                format!("{} 11:30:00", date).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();
            let n_start = NaiveDateTime::parse_from_str(
                format!("{} 13:00:00", date).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();
            let n_end = NaiveDateTime::parse_from_str(
                format!("{} 15:00:00", date).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();

            let idx = if *n > m_start && *n <= m_end {
                0
            } else if *n > m_end && *n <= n_start {
                1
            } else if *n > n_start && *n <= n_end {
                2
            } else {
                3
            };

            for i in 0..=idx {
                if !self.base_event[i] {
                    return self.fire_base_event(i);
                }
            }
        }
        Ok(None)
    }
    fn add_codes(&mut self, codes: &Vec<String>) {
        let codes: Vec<_> = codes
            .iter()
            .filter(|code| !self.codes.contains(code))
            .map(|code| code.clone())
            .collect();
        if !codes.is_empty() {
            self.codes.extend(codes.into_iter());
        }
    }
    fn reset_trade_date(&mut self, n: &NaiveDateTime) {
        if self.trade_date.is_none() || self.trade_date.unwrap() != n.date() {
            for i in 0..4 {
                self.base_event[i] = false;
            }

            let date = n.date();

            self.trade_date = Some(date);
        }
    }
}

struct BacktestQuotation {}

impl BacktestQuotation {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Quotation for BacktestQuotation {
    async fn subscribe(&mut self, codes: &Vec<String>) {}
    async fn fetch(&mut self, codes: &Option<Vec<String>>) -> Result<Option<QuotEvent>> {
        todo!()
    }
}

struct RealtimeQuotation {
    quotation: MyQuotation,
}

impl RealtimeQuotation {
    fn new(opts: QuotOpts) -> Self {
        Self {
            quotation: MyQuotation {
                opts: opts,
                codes: vec![],
                is_start: false,
                is_end: false,
                trade_date: None,
                base_event: [false; 4],
            },
        }
    }
}

#[async_trait]
impl Quotation for RealtimeQuotation {
    async fn subscribe(&mut self, codes: &Vec<String>) {
        self.add_codes(&codes);
    }
    async fn fetch(&mut self, codes: &Option<Vec<String>>) -> Result<Option<QuotEvent>> {
        if let Some(new_codes) = codes {
            self.add_codes(new_codes);
        }

        if !self.is_start {
            self.is_start = true;
            return Ok(Some(QuotEvent::Start));
        }

        let n = Local::now().naive_local();
        if !fetch_is_trade_date(&n.date())
            .await
            .map_err(|e| Error::Custom(e.to_string()))?
        {
            return Ok(None);
        }

        self.reset_trade_date(&n);

        let be = self.get_base_event(&n)?;
        if let Some(e) = be {
            return Ok(Some(e));
        }

        if !self.is_trading() || self.codes.is_empty() {
            return Ok(None);
        }

        let quot = fetch_rt_quot(&self.codes)
            .await
            .map_err(|e| Error::Custom(format!("{}", e.to_string())))?;

        Ok(Some(QuotEvent::Quot(quot)))
    }
}

impl Deref for RealtimeQuotation {
    type Target = MyQuotation;

    fn deref(&self) -> &Self::Target {
        &self.quotation
    }
}

impl DerefMut for RealtimeQuotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quotation
    }
}
