use crate::comm::EastBar;
use crate::{AdjustFactor, Result, HTTP_CMM_HEADER};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Timelike};
use rwqcmm::{Bar, BarFreq};
use std::ops::Add;

use super::trade_date::fetch_prev_trade_date;

// pub(crate) fn block_client() -> reqwest::blocking::Client {
//     reqwest::blocking::ClientBuilder::new()
//         .cookie_store(true)
//         .default_headers(HTTP_CMM_HEADER.to_owned())
//         .build()
//         .unwrap()
// }

pub(crate) fn async_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(HTTP_CMM_HEADER.to_owned())
        .build()
        .unwrap()
}

pub(crate) fn to_bar_ds(name: Option<&str>, bars: Vec<Bar>) -> (String, Option<Vec<Bar>>) {
    let bars_len = bars.len();
    let mut stock_name = "".to_owned();
    if let Some(s_name) = name {
        stock_name = s_name.to_owned()
    } else {
        if bars_len > 0 {
            stock_name = (&bars[0]).name.clone()
        }
    }
    let bars = if bars_len > 0 { Some(bars) } else { None };
    (stock_name, bars)
}

pub(crate) async fn fetch_bar(
    client: &reqwest::Client,
    market_code: &str,
    orig_code: &str,
    freq: BarFreq,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    skip_rt: bool,
) -> Result<Vec<Bar>> {
    let mut first_date: Option<i32> = None;
    let mut start_str = "0".to_owned();
    if let Some(s) = &start {
        let prev = fetch_prev_trade_date(s).await?;
        start_str = format!("{}", prev);
        first_date = Some(prev);
    }

    let mut n = Local::now().naive_local();
    if skip_rt {
        // 当日的不准
        if n.hour() < 15 && matches!(freq, BarFreq::Daily) {
            let mut minus_day = true;
            if let Some(e) = end {
                if n.date() > e {
                    minus_day = false;
                    let d_str = format!("{} 00:00:00", e.format("%Y%m%d"));
                    n = NaiveDateTime::parse_from_str(&d_str, "%Y%m%d %H:%M:%S").unwrap();
                }
            }
            if minus_day {
                n = n.add(Duration::days(-1))
            }
        }
    }

    let mut data = Vec::new();
    if let Some(s) = &start {
        if s > &n.date() {
            return Ok(data);
        }
    }
    let end_str = n.format("%Y%m%d").to_string();
    let kline_type = if matches!(freq, BarFreq::LooseDaily) {
        BarFreq::Daily as i32
    } else {
        freq as i32
    };

    for fq_type in vec![AdjustFactor::NFQ, AdjustFactor::HFQ] {
        let req_url = format!(
            "https://push2his.eastmoney.com/api/qt/stock/kline/get?\
        fields1=f1%2Cf2%2Cf3%2Cf4%2Cf5%2Cf6&\
        fields2=f51%2Cf52%2Cf53%2Cf54%2Cf55%2Cf56%2Cf57%2Cf58%2Cf59%2Cf60%2Cf61&\
        ut=7eea3edcaed734bea9cbfc24409ed989&klt={kline_type}&fqt={fq_type}&secid={market_code}&\
        beg={start}&end={end}&_=1667196199286",
            kline_type = kline_type,
            fq_type = fq_type as i32,
            market_code = market_code,
            start = start_str,
            end = end_str
        );

        let resp = client.get(req_url).send().await?.text().await?;

        let mut pre_item: Option<Bar> = None;
        let json: EastBar = serde_json::from_str(&resp)?;
        let tmp_bars: Option<Vec<_>> = if let Some(data) = json.data {
            let tmp_vec: Vec<_> = data
                .klines
                .iter()
                .map(|item| {
                    let s: Vec<&str> = item.split(",").collect();
                    debug_assert!(s.len() == 11);
                    let trade_date = &s.get(0).unwrap()[..];
                    let trade_date = if matches!(freq, BarFreq::Daily)
                        || matches!(freq, BarFreq::LooseDaily)
                        || matches!(freq, BarFreq::Weekly)
                        || matches!(freq, BarFreq::Monthly)
                    {
                        format!("{} 00:00:00", trade_date)
                    } else {
                        format!("{}:00", trade_date)
                    };
                    let volume = s.get(5).unwrap().parse().unwrap();
                    let amount = s.get(6).unwrap().parse().unwrap();
                    let (volume_chg_pct, amount_chg_pct) = if let Some(item) = &pre_item {
                        (
                            (((volume as i64 - item.volume as i64) * 100) as f64
                                / item.volume as f64) as f32,
                            ((amount - item.amount) * 100.0 / item.amount) as f32,
                        )
                    } else {
                        (0.0, 0.0)
                    };

                    let bar = Bar {
                        code: orig_code.to_owned(),
                        name: data.name.to_owned(),
                        trade_date: NaiveDateTime::parse_from_str(&trade_date, "%Y-%m-%d %H:%M:%S")
                            .unwrap(),
                        open: s.get(1).unwrap().parse().unwrap(),
                        close: s.get(2).unwrap().parse().unwrap(),
                        high: s.get(3).unwrap().parse().unwrap(),
                        low: s.get(4).unwrap().parse().unwrap(),
                        volume,
                        amount,
                        volume_chg_pct,
                        amount_chg_pct,
                        turnover: s.get(10).unwrap().parse().unwrap(),
                        chg_pct: s.get(8).unwrap().parse().unwrap(),
                        hfq_factor: 1.0,
                    };
                    pre_item = Some(bar.clone());
                    bar
                })
                .collect();

            Some(tmp_vec)
        } else {
            None
        };
        if let Some(bars) = tmp_bars {
            if matches!(fq_type, AdjustFactor::NFQ) {
                data.extend(bars.into_iter());
            } else if matches!(fq_type, AdjustFactor::HFQ) {
                if data.len() == bars.len() {
                    data.iter_mut()
                        .zip(bars.iter())
                        .for_each(|(t_data, t_bar)| {
                            t_data.hfq_factor = t_bar.close / t_data.close;
                        })
                }
            }
        }
    }
    if let Some(first_date) = first_date {
        if data.len() > 0 {
            let mut skip = 0;
            for i in 0..data.len() {
                let first = data.get(i).unwrap();
                let (y, m, d) = (
                    first.trade_date.year(),
                    first.trade_date.month(),
                    first.trade_date.day(),
                );
                let date = y * 10000 + m as i32 * 100 + d as i32;
                if first_date == date {
                    //data = data.into_iter().skip(1).collect();
                    skip = skip + 1;
                } else {
                    break;
                }
            }
            if skip > 0 {
                data = data.into_iter().skip(skip).collect();
            }
        }
    }
    Ok(data)
}
