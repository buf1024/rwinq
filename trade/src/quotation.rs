use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::{Local, NaiveDate, NaiveDateTime};
use rwqdata::{fetch_is_trade_date, fetch_rt_quot, BarFreq, Quot, RtQuot};
use rwqtradecmm::{QuotEvent, QuotOpts};

#[async_trait]
pub trait Quotation {
    async fn subscribe(&mut self, codes: &Vec<String>);
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
    quots: BTreeMap<u64, RtQuot>,
    index: usize,
    iter: Vec<u64>,
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
    async fn subscribe(&mut self, codes: &Vec<String>) {
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
            
            for code in new_codes {
                let r = if m < FREQ_1D {
                    self.fetcher
                        .fetch_stock_minute(code.as_str(), m / 60)
                        .await
                        .with_context(|| "fetch stock minute error")?
                } else {
                    let mut q_data = Vec::new();
                    if self.db.is_some() {
                        let db = self.db.as_ref().unwrap();
                        let filter = match (self.opts.start_date, self.opts.end_date) {
                            (None, None) => doc! {"code": code.as_str()},
                            (None, Some(end)) => {
                                let s = end.and_hms(0, 0, 0);
                                let s = Local.from_local_datetime(&s).unwrap();
                                doc! {
                                   "code": code.as_str(),
                                   "trade_date": {"$lte": s}
                                }
                            }
                            (Some(start), None) => {
                                let s = start.and_hms(0, 0, 0);
                                let s = Local.from_local_datetime(&s).unwrap();
                                doc! {
                                   "code": code.as_str(),
                                   "trade_date": {"$gte": s}
                                }
                            }
                            (Some(start), Some(end)) => {
                                let s = start.and_hms(0, 0, 0);
                                let s = Utc.from_local_datetime(&s).unwrap();

                                let e = end.and_hms(0, 0, 0);
                                let e = Utc.from_local_datetime(&e).unwrap();
                                doc! {
                                   "code": code.as_str(),
                                   "trade_date": {"$gte": s, "$lte": e},
                                }
                            }
                        };
                        let opts = FindOptions::builder().sort(doc! {"trade_date": 1}).build();
                        if is_index(code.as_str()) {
                            let mut cursor = db
                                .get_coll::<IndexDaily>("index_daily")?
                                .find(filter, opts)
                                .await
                                .with_context(|| "query index_daily failed")?;
                            while let Some(item) = cursor
                                .try_next()
                                .await
                                .with_context(|| "query index_daily failed")?
                            {
                                let time = item
                                    .trade_date
                                    .to_chrono()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string();
                                let s_bar = StockBar {
                                    time,
                                    open: item.open,
                                    high: item.high,
                                    low: item.low,
                                    close: item.close,
                                    vol: item.volume,
                                };
                                q_data.push(s_bar)
                            }
                        } else {
                            let mut cursor = db
                                .get_coll::<StockDaily>("stock_daily")?
                                .find(filter, opts)
                                .await
                                .with_context(|| "query stock_daily failed")?;
                            while let Some(item) = cursor
                                .try_next()
                                .await
                                .with_context(|| "query stock_daily failed")?
                            {
                                let time = item
                                    .trade_date
                                    .to_chrono()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string();
                                let s_bar = StockBar {
                                    time,
                                    open: item.open,
                                    high: item.high,
                                    low: item.low,
                                    close: item.close,
                                    vol: item.volume as u64,
                                };
                                q_data.push(s_bar)
                            }
                        }
                    }
                    q_data
                };
                for bar in r.iter() {
                    let t = NaiveDateTime::parse_from_str(bar.time.as_str(), "%Y-%m-%d %H:%M:%S")
                        .with_context(|| "parse time error")?;
                    let t = t.timestamp();
                    if !self.bar_list.contains_key(&(t as u64)) {
                        self.bar_list.insert(t as u64, RtQuotBar::new());
                    }
                    let q_bar = self.bar_list.get_mut(&(t as u64)).unwrap();

                    if !q_bar.contains_key(&code) {
                        let rt_q = QuotBar {
                            frequency,
                            open: bar.open,
                            high: bar.high,
                            low: bar.close,
                            close: bar.high,
                            start: NaiveDateTime::from_timestamp(t - (frequency as i64), 0)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string(),
                            end: bar.time.clone(),
                            quot: Quot {
                                code: code.clone(),
                                open: bar.open,
                                now: bar.close,
                                high: bar.high,
                                low: bar.low,
                                buy: bar.close,
                                sell: bar.close,
                                vol: bar.vol,
                                date: NaiveDateTime::parse_from_str(
                                    &bar.time[..],
                                    "%Y-%m-%d %H:%M:%S",
                                )
                                .unwrap()
                                .date()
                                .format("%Y-%m-%d")
                                .to_string(),
                                time: bar.time.clone(),
                                ..Default::default()
                            },
                        };
                        q_bar.insert(code.clone(), rt_q);
                    }
                }
            }
            self.iter_vec.clear();
            self.iter_vec.extend(self.bar_list.keys());
        }
        // Ok(())
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
    async fn subscribe(&mut self, codes: &Vec<String>) {
        self.add_codes(&codes);
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
