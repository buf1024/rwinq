use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use rwqdata::{fetch_is_trade_date, fetch_rt_quot, fetch_stock_bar, BarFreq, Quot, RtQuot};
use rwqtradecmm::{QuotEvent, QuotOpts};

#[async_trait]
pub trait Quotation {
    async fn subscribe(&mut self, codes: &Vec<String>) -> Result<()>;
    async fn fetch(&mut self, codes: &Option<Vec<String>>) -> Result<Option<QuotEvent>>;
}

pub fn backtest(opts: QuotOpts) -> Box<dyn Quotation> {
    Box::new(BacktestQuotation::new(opts))
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
    fn test_base_event(&mut self, n: &NaiveDateTime) -> Result<Option<QuotEvent>> {
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

    async fn get_base_event(
        &mut self,
        codes: &Option<Vec<String>>,
        n: &NaiveDateTime,
    ) -> Result<Option<QuotEvent>> {
        if let Some(new_codes) = codes {
            self.add_codes(new_codes);
        }

        if !self.is_start {
            self.is_start = true;
            return Ok(Some(QuotEvent::Start));
        }

        if !fetch_is_trade_date(&n.date())
            .await
            .map_err(|e| Error::Custom(e.to_string()))?
        {
            return Ok(None);
        }
        self.reset_trade_date(&n);

        let be = self.test_base_event(&n)?;
        if let Some(e) = be {
            return Ok(Some(e));
        }

        if !self.is_trading() || self.codes.is_empty() {
            return Ok(None);
        }
        return Ok(None);
    }
}

struct BacktestQuotation {
    quotation: MyQuotation,
    quots: BTreeMap<i64, RtQuot>,
    index: usize,
    iter: Vec<i64>,
    freq: Vec<i64>,
}

impl BacktestQuotation {
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
            quots: BTreeMap::new(),
            index: 0,
            iter: Vec::new(),
            freq: vec![
                BarFreq::Min1.to_seconds(),
                BarFreq::Min5.to_seconds(),
                BarFreq::Min15.to_seconds(),
                BarFreq::Min30.to_seconds(),
                BarFreq::Min60.to_seconds(),
                BarFreq::Daily.to_seconds(),
            ],
        }
    }
    fn get_freq(&self) -> Option<BarFreq> {
        if self.opts.freq == BarFreq::Min1.to_seconds() {
            Some(BarFreq::Min1)
        } else if self.opts.freq == BarFreq::Min5.to_seconds() {
            Some(BarFreq::Min5)
        } else if self.opts.freq == BarFreq::Min15.to_seconds() {
            Some(BarFreq::Min15)
        } else if self.opts.freq == BarFreq::Min30.to_seconds() {
            Some(BarFreq::Min30)
        } else if self.opts.freq == BarFreq::Min60.to_seconds() {
            Some(BarFreq::Min60)
        } else if self.opts.freq == BarFreq::Min5.to_seconds() {
            Some(BarFreq::Daily)
        } else {
            None
        }
    }
}
#[async_trait]
impl Quotation for BacktestQuotation {
    async fn subscribe(&mut self, codes: &Vec<String>) -> Result<()> {
        let new_codes: Vec<_> = codes
            .iter()
            .filter(|code| !self.codes.contains(code))
            .map(|code| code.clone())
            .collect();
        if !new_codes.is_empty() {
            self.codes.extend(new_codes.clone().into_iter());
        }

        if self.freq.contains(&self.opts.freq) && !new_codes.is_empty() {
            let freq = self.get_freq().unwrap();
            let start = match self.opts.start_date.as_ref() {
                Some(start) => {
                    let mut start_date;
                    if self.iter.len() > 0 && self.index < self.iter.len() {
                        start_date = Some(
                            Utc.timestamp_opt(self.iter[self.index] as i64, 0)
                                .unwrap()
                                .naive_local()
                                .date(),
                        );
                    } else {
                        return Ok(());
                    }
                    if start_date.is_none() {
                        start_date = Some(start.date())
                    }
                    start_date
                }
                None => None,
            };
            let end = self.opts.end_date.as_ref().map(|end| end.date());

            for code in new_codes {
                let bars = fetch_stock_bar(&code, None, Some(freq), start, end, true)
                    .await
                    .map_err(|e| Error::Custom(format!("{}", e.to_string())))?;
                if bars.bars.is_none() {
                    continue;
                }
                let bars = bars.bars.unwrap();

                for bar in bars.iter() {
                    let ts = bar.trade_date.timestamp();
                    if !self.quots.contains_key(&ts) {
                        self.quots.insert(ts, RtQuot::new());
                    }
                    let quot = self.quots.get_mut(&ts).unwrap();

                    if !quot.contains_key(&code) {
                        let new_quot = Quot {
                            code: code.clone(),
                            name: bar.name.clone(),
                            open: bar.open,
                            last_close: bar.close / (1.0 + bar.chg_pct / 100.0),
                            now: bar.close,
                            high: bar.high,
                            low: bar.low,
                            buy: bar.close,
                            sell: bar.close,
                            volume: bar.volume,
                            amount: bar.amount,
                            bid: Default::default(),
                            ask: Default::default(),
                            time: Default::default(),
                            chg: bar.chg_pct / 100.0 * bar.close,
                            chg_pct: bar.chg_pct,
                            turnover: bar.turnover,
                            total_value: 0.0,
                            currency_value: 0.0,
                            is_trading: true,
                            freq_open: bar.open,
                            freq_high: bar.high,
                            freq_low: bar.low,
                            freq_chg: bar.chg_pct / 100.0 * bar.close,
                            freq_chg_pct: bar.chg_pct,
                            freq_time: bar.trade_date,
                        };
                        quot.insert(code.clone(), new_quot);
                    }
                }
            }
            self.iter.extend(self.quots.keys());
        }
        Ok(())
    }
    async fn fetch(&mut self, codes: &Option<Vec<String>>) -> Result<Option<QuotEvent>> {
        todo!()
    }
}

impl Deref for BacktestQuotation {
    type Target = MyQuotation;

    fn deref(&self) -> &Self::Target {
        &self.quotation
    }
}

impl DerefMut for BacktestQuotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quotation
    }
}

struct RealtimeQuotation {
    quotation: MyQuotation,
    quot: RtQuot,
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
            quot: RtQuot::new(),
        }
    }

    fn to_freq_quot(&mut self, n: &NaiveDateTime, quot: &RtQuot) -> Option<RtQuot> {
        let (mut is_ready, mut is_test) = (false, false);
        for (code, q) in quot.iter() {
            let (freq_open, freq_high, freq_low, freq_chg, freq_chg_pct, freq_time) =
                match self.quot.get(code) {
                    Some(cq) => {
                        let mut freq_high = cq.freq_high;
                        if cq.freq_high < q.now {
                            freq_high = q.now;
                        }
                        let mut freq_low = cq.freq_low;
                        if cq.low > q.now {
                            freq_low = q.now;
                        }
                        let freq_chg = q.now - q.freq_open;
                        let freq_chg_pct = freq_chg / q.freq_open * 100.0;
                        (
                            cq.freq_open,
                            freq_high,
                            freq_low,
                            freq_chg,
                            freq_chg_pct,
                            cq.freq_time,
                        )
                    }
                    None => (q.now, q.now, q.now, 0.0, 0.0, *n),
                };
            self.quot.insert(
                code.clone(),
                Quot {
                    freq_open,
                    freq_high,
                    freq_low,
                    freq_chg,
                    freq_chg_pct,
                    freq_time,
                    ..q.clone()
                },
            );

            if !is_test {
                is_test = true;
                let start = freq_time.timestamp();
                let end = n.timestamp();
                if end - start >= self.opts.freq {
                    is_ready = true;
                }
            }
        }

        if is_ready {
            let quot = self.quot.clone();
            self.quot.clear();
            return Some(quot);
        }

        None
    }
}

#[async_trait]
impl Quotation for RealtimeQuotation {
    async fn subscribe(&mut self, codes: &Vec<String>) -> Result<()> {
        self.add_codes(&codes);
        Ok(())
    }
    async fn fetch(&mut self, codes: &Option<Vec<String>>) -> Result<Option<QuotEvent>> {
        let n = Local::now().naive_local();

        let base_event = self.get_base_event(codes, &n).await?;
        if let Some(event) = base_event {
            return Ok(Some(event));
        }

        let quot = self.to_freq_quot(
            &n,
            &fetch_rt_quot(&self.codes)
                .await
                .map_err(|e| Error::Custom(format!("{}", e.to_string())))?,
        );

        let event = quot.map(|quot| QuotEvent::Quot(quot));

        Ok(event)
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
