use crate::comm::EastBar;
use crate::{AdjustFactor, Error, Result, XuQiuRtQuot, HTTP_CMM_HEADER};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};
use regex::Regex;
use rwqcmm::{Bar, BarFreq, Quot, QuotSn, QuotXq, RtQuot, RtQuotSn, RtQuotXq};
use std::ops::Add;
use tracing::{debug, instrument};

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

#[instrument(skip(client))]
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
    let mut end_set = false;
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
            end_set = true;
        }
    }
    if !end_set {
        if let Some(e) = end {
            let d_str = format!("{} 00:00:00", e.format("%Y%m%d"));
            n = NaiveDateTime::parse_from_str(&d_str, "%Y%m%d %H:%M:%S").unwrap();
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

        debug!(request = req_url);

        let resp = client.get(req_url).send().await?.text().await?;
        let mut pre_item: Option<Bar> = None;
        let json: EastBar = serde_json::from_str(&resp)?;
        let tmp_bars: Option<Vec<_>> = if let Some(data) = json.data {
            debug!(bar = data.klines.len());
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
            debug!(bar = "None");
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

/// 雪球实时行情
pub async fn fetch_rt_quot_xq(code: &Vec<String>) -> Result<RtQuotXq> {
    let codes = code
        .iter()
        .map(|s| s.to_uppercase())
        .collect::<Vec<_>>()
        .join(",");

    let req_url = format!(
        "https://stock.xueqiu.com/v5/stock/realtime/quotec.json?\
    symbol={codes}",
        codes = codes
    );
    let client = async_client();

    let resp = client.get(req_url).send().await?.text().await?;

    let json: XuQiuRtQuot = serde_json::from_str(&resp)?;
    let data = json
        .data
        .ok_or(Error::Custom("Error fetch quotation".to_string()))?;

    let data: Vec<_> = data
        .iter()
        .map(|item| {
            let code = item.symbol.to_lowercase();
            let time = Local
                .timestamp_opt(item.timestamp / 1000, 0)
                .unwrap()
                .naive_local();
            let mut is_trading = false;
            let t = time.time();
            let ms = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
            let me = NaiveTime::from_hms_opt(11, 30, 0).unwrap();
            let ns = NaiveTime::from_hms_opt(13, 0, 0).unwrap();
            let ne = NaiveTime::from_hms_opt(15, 0, 0).unwrap();
            if (t > ms && t < me) || (t > ns && t < ne) {
                is_trading = true;
            }
            (
                code.clone(),
                QuotXq {
                    code,
                    time,
                    last_close: item.last_close,
                    open: item.open,
                    high: item.high,
                    low: item.low,
                    now: item.now,
                    chg: item.chg,
                    chg_pct: item.percent,
                    volume: item.volume,
                    amount: item.amount,
                    turnover: item.turnover_rate.unwrap_or_default(),
                    total_value: item.market_capital.unwrap_or_default(),
                    currency_value: item.float_market_capital.unwrap_or_default(),
                    is_trading: is_trading,
                },
            )
        })
        .collect();

    Ok(data.into_iter().collect())
}

/// 新浪实时行情
pub async fn fetch_rt_quot_sn(code: &Vec<String>) -> Result<RtQuotSn> {
    let req_url = format!("http://hq.sinajs.cn/?format=text&list={}", code.join(","));
    let client: reqwest::Client = async_client();

    let resp = client
        .get(req_url)
        .header("Referer", "https://finance.sina.com.cn/")
        .send()
        .await?
        .text()
        .await?;

    let mut re_str = String::from(r"(\w+)=([^\s][^,]+?)");
    for _ in 0..29 {
        re_str.push_str(r",([\.\d]+)")
    }
    for _ in 0..2 {
        re_str.push_str(r",([-\.\d:]+)");
    }
    let regex = Regex::new(re_str.as_str()).unwrap();

    if !regex.is_match(resp.as_str()) {
        return Err(Error::Custom("Sina response data error!".into()));
    }

    let mut rq = RtQuotSn::new();
    for cap in regex.captures_iter(resp.as_str()) {
        let date: NaiveDate = cap[32]
            .parse()
            .map_err(|_| Error::Custom(format!("Parse naive_date error: {}!", &cap[32])))?;
        let time: NaiveTime = cap[33]
            .parse()
            .map_err(|_| Error::Custom(format!("Parse naive_time error: {}!", &cap[33])))?;

        let time = NaiveDateTime::new(date, time);

        let q = QuotSn {
            code: String::from(&cap[1]),
            name: String::from(&cap[2]),
            open: cap[3]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse open error: {}!", &cap[3])))?,
            last_close: cap[4]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse pre_close error: {}!", &cap[4])))?,
            now: cap[5]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse now error: {}!", &cap[5])))?,
            high: cap[6]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse high error: {}!", &cap[6])))?,
            low: cap[7]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse low error: {}!", &cap[7])))?,
            buy: cap[8]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse buy error: {}!", &cap[8])))?,
            sell: cap[9]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse sell error: {}!", &cap[9])))?,
            volume: cap[10]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse vol error: {}!", &cap[10])))?,
            amount: cap[11]
                .parse()
                .map_err(|_| Error::Custom(format!("Parse amount error: {}!", &cap[11])))?,
            bid: (
                (
                    cap[12]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid[1] error: {}!", &cap[12])))?,
                    cap[13]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid[1] error: {}!", &cap[13])))?,
                ),
                (
                    cap[14]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[14])))?,
                    cap[15]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[14])))?,
                ),
                (
                    cap[16]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[16])))?,
                    cap[17]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[17])))?,
                ),
                (
                    cap[18]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[18])))?,
                    cap[19]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[19])))?,
                ),
                (
                    cap[20]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[20])))?,
                    cap[21]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse bid error: {}!", &cap[21])))?,
                ),
            ),
            ask: (
                (
                    cap[22]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[22])))?,
                    cap[23]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[23])))?,
                ),
                (
                    cap[24]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[24])))?,
                    cap[25]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[25])))?,
                ),
                (
                    cap[26]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[26])))?,
                    cap[27]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[27])))?,
                ),
                (
                    cap[28]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[28])))?,
                    cap[29]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[29])))?,
                ),
                (
                    cap[30]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[30])))?,
                    cap[31]
                        .parse()
                        .map_err(|_| Error::Custom(format!("Parse ask error: {}!", &cap[31])))?,
                ),
            ),
            time,
        };
        rq.insert(q.code.clone(), q);
    }
    Ok(rq)
}

pub async fn fetch_rt_quot(code: &Vec<String>) -> Result<RtQuot> {
    let (xq, sn) = tokio::join!(fetch_rt_quot_xq(code), fetch_rt_quot_sn(code));
    let (mxq, msn) = (xq?, sn?);

    if mxq.len() != msn.len() {
        return Err(Error::Custom(String::from("fail to fetch quot")));
    }

    let rt_quot = msn
        .into_iter()
        .map(|(k, sn)| {
            let xq = mxq.get(&k).unwrap();
            let quot = Quot {
                chg: xq.chg,
                chg_pct: xq.chg_pct,
                turnover: xq.turnover,
                total_value: xq.total_value,
                currency_value: xq.currency_value,
                is_trading: xq.is_trading,
                code: sn.code,
                name: sn.name,
                open: sn.open,
                last_close: sn.last_close,
                now: sn.now,
                high: sn.high,
                low: sn.low,
                buy: sn.buy,
                sell: sn.sell,
                volume: sn.volume,
                amount: sn.amount,
                bid: sn.bid,
                ask: sn.ask,
                time: sn.time,
            };
            (k.clone(), quot)
        })
        .collect();
    Ok(rt_quot)
}

#[cfg(test)]
mod tests {
    use crate::fetch_rt_quot;

    #[test]
    fn test_fetch_rt_quot() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                //上海深圳股票
                let codes = vec!["sz000001".into(), "sz002805".into(), "sh600887".into()];
                let data = fetch_rt_quot(&codes).await;
                println!("{:?}", data);
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);
                data.iter().for_each(|(key, val)| {
                    println!("data[{}]={:?}", key, val);
                });

                // 北京股票
                let codes = vec!["bj832089".into()];
                let data = fetch_rt_quot(&codes).await;
                println!("{:?}", data);
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);
                data.iter().for_each(|(key, val)| {
                    println!("data[{}]={:?}", key, val);
                });

                // ETF
                let codes = vec!["sz159949".into()];
                let data = fetch_rt_quot(&codes).await;
                println!("{:?}", data);
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);
                data.iter().for_each(|(key, val)| {
                    println!("data[{}]={:?}", key, val);
                });

                // 可转债
                let codes = vec!["sz128030".into()];
                let data = fetch_rt_quot(&codes).await;
                println!("{:?}", data);
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);
                data.iter().for_each(|(key, val)| {
                    println!("data[{}]={:?}", key, val);
                });
            })
    }
}
