use crate::comm::{async_client, fetch_bar, XueQiuBar};
use crate::fund::hi_fund_info::EastFundNet;
use crate::fund::FundFetch;
use crate::util::to_std_code;
use crate::{Error, HeaderValue, Market, MarketType, Result, HTTP_CMM_HEADER};
use async_trait::async_trait;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, TimeZone};
use hiq_common::{Bar, BarFreq, FundBar, FundInfo, FundNet};
use reqwest::header::REFERER;
use reqwest::Client;
use std::ops::Add;

pub struct HiqFundFetch {
    client: Client,
}

impl HiqFundFetch {
    pub fn new() -> Self {
        Self {
            client: async_client(),
        }
    }

    pub async fn fetch_fund_bar_xq(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<FundBar> {
        let code = code.to_uppercase();
        let name = name.unwrap_or("");
        let mut start = start.map_or(
            NaiveDateTime::parse_from_str("2010-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            |d| d.and_hms_opt(0, 0, 0).unwrap(),
        );
        let end = end.map_or(Local::now().naive_local(), |d| {
            d.and_hms_opt(0, 0, 0).unwrap()
        });

        let mut data = Vec::new();

        // prepare cookie
        self.client.get("https://xueqiu.com/hq").send().await?;
        while start <= end {
            let timestamp = start.timestamp() * 1000;
            let req_url = format!(
                "https://stock.xueqiu.com/v5/stock/chart/kline.json?\
            symbol={code}&begin={timestamp}&period=day&type=before&count=100&indicator=kline",
                code = code,
                timestamp = timestamp
            );

            let resp = self.client.get(req_url).send().await?.text().await?;
            let json: XueQiuBar = serde_json::from_str(&resp)?;

            if let Some(result) = json.data {
                let tmp_vec: Vec<_> = result
                    .item
                    .iter()
                    .map(|item| {
                        // ["timestamp","volume","open","high","low","close","chg","percent","turnoverrate","amount","volume_post","amount_post"]
                        let trade_date: NaiveDateTime =
                            Local.timestamp_opt(item.0 / 1000, 0).unwrap().naive_local();
                        Bar {
                            code: result.code[2..].to_owned(),
                            name: name.to_owned(),
                            trade_date,
                            open: item.2.unwrap_or(0.0),
                            close: item.5.unwrap_or(0.0),
                            high: item.3.unwrap_or(0.0),
                            low: item.4.unwrap_or(0.0),
                            volume: item.1.unwrap_or(0),
                            amount: item.9.unwrap_or(0.0),
                            turnover: item.8.unwrap_or(0.0),
                            chg_pct: item.6.unwrap_or(0.0),
                            hfq_factor: 1.0,
                        }
                    })
                    .filter(|item| item.trade_date <= end)
                    .collect();
                if tmp_vec.is_empty() {
                    break;
                }
                let last = tmp_vec[tmp_vec.len() - 1].trade_date.clone();

                start = last.add(Duration::days(1));
                data.extend(tmp_vec.into_iter());
            } else {
                break;
            }
        }

        Ok(FundBar {
            code: code.to_lowercase(),
            name: name.to_string(),
            freq: BarFreq::Daily,
            bars: if data.len() > 0 { Some(data) } else { None },
        })
    }
}

#[async_trait]
impl FundFetch for HiqFundFetch {
    /// etf基金基本信息
    async fn fetch_fund_info(&self) -> Result<Vec<FundInfo>> {
        let req_url = format!("http://fund.eastmoney.com/js/fundcode_search.js?v=20130718.js");

        let resp = self.client.get(req_url).send().await?.text().await?;

        let index = resp.find("[").ok_or(Error::Custom("Invalid response"))?;
        let resp = &resp[index..resp.len() - 1];
        let json = serde_json::from_str::<Vec<Vec<&str>>>(resp)?;

        let data: Vec<_> = json
            .iter()
            .filter(|item| {
                item.len() == 5
                    && (item[2].to_uppercase().contains("ETF") && !item[2].contains("联接"))
            })
            .map(|item| {
                // ["000001","HXCZHH","华夏成长混合","混合型-灵活","HUAXIACHENGZHANGHUNHE"]
                FundInfo {
                    code: to_std_code(MarketType::Fund, item[0]),
                    name: item[2].to_owned(),
                }
            })
            .collect();

        Ok(data)
    }
    /// etf基金净值
    async fn fetch_fund_net(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<FundNet>> {
        let name = name.unwrap_or("");
        let start = start.unwrap_or(NaiveDate::parse_from_str("2010-01-01", "%Y-%m-%d").unwrap());
        let now = Local::now();
        let end = end.unwrap_or(now.date_naive());
        let req_url = format!(
            "https://api.fund.eastmoney.com/f10/lsjz?\
        fundCode={code}&pageIndex=1&pageSize=10000&startDate={start}&endDate={end}&_={timestamp}",
            code = &code[2..],
            start = start,
            end = end,
            timestamp = now.timestamp()
        );

        let mut headers = HTTP_CMM_HEADER.to_owned();
        let referer = format!("http://fundf10.eastmoney.com/jjjz_{code}.html", code = code);
        headers.insert(REFERER, HeaderValue::from_str(&referer).unwrap());
        let resp = self
            .client
            .get(req_url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let json: EastFundNet = serde_json::from_str(&resp)?;

        let mut data = Vec::new();
        if let Some(js_data) = json.data {
            data = js_data
                .list
                .iter()
                .map(|item| FundNet {
                    code: code.to_string(),
                    name: name.to_string(),
                    trade_date: NaiveDate::parse_from_str(item.trade_date, "%Y-%m-%d").unwrap(),
                    net: item.net.parse().unwrap_or(0.0),
                    net_acc: item.net_acc.parse().unwrap_or(0.0),
                    chg_pct: item.chg_pct.parse().unwrap_or(0.0),
                    apply_status: item.apply_status.to_string(),
                    redeem_status: item.redeem_status.to_string(),
                })
                .collect();
        }

        Ok(data)
    }
    /// etf基金k线数据
    async fn fetch_fund_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<FundBar> {
        let market_code = if code.starts_with("sz") {
            format!("{}.{}", Market::SZ as i32, &code[2..])
        } else {
            format!("{}.{}", Market::SH as i32, &code[2..])
        };
        let freq = if freq.is_none() {
            BarFreq::Daily
        } else {
            freq.unwrap()
        };

        let bars = fetch_bar(&self.client, &market_code, code, freq, start, end).await?;
        let bond_bar = FundBar {
            code: code.to_owned(),
            name: name.unwrap_or("").to_owned(),
            freq,
            bars: if bars.len() > 0 { Some(bars) } else { None },
        };
        Ok(bond_bar)
    }
}

#[cfg(test)]
mod tests {
    use crate::fund::{FundFetch, HiqFundFetch};
    use crate::util::to_std_code;
    use crate::MarketType;
    use chrono::NaiveDate;

    #[test]
    fn test_fetch_fund_info() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = HiqFundFetch::new();

                let data = fetch.fetch_fund_info().await;

                assert!(data.is_ok());
                let data = data.unwrap();
                println!("len={}", data.len());
                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);

                let data = data
                    .iter()
                    .find(|item| item.name.to_uppercase().contains("QDII"));
                assert!(data.is_some());
                let data = data.unwrap();
                println!("data[QDII]={:?}", data);
            })
    }

    #[test]
    fn test_fetch_fund_net() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = HiqFundFetch::new();

                let data = fetch.fetch_fund_net(
                    &to_std_code(MarketType::Fund, "159915"),
                    None,
                    Some(NaiveDate::parse_from_str("2022-11-07", "%Y-%m-%d").unwrap()),
                    Some(NaiveDate::parse_from_str("2022-11-10", "%Y-%m-%d").unwrap()),
                ).await;
                if data.is_err() {
                    println!("error: {:?}", data);
                }
                assert!(data.is_ok());
                let data = data.unwrap();

                println!("data={:?}", data);
            })
    }

    #[test]
    fn test_fetch_fund_bar_xq() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = HiqFundFetch::new();

                let data = fetch.fetch_fund_bar_xq(
                    &to_std_code(MarketType::Fund, "159915"),
                    None,
                    Some(NaiveDate::parse_from_str("2022-11-07", "%Y-%m-%d").unwrap()),
                    Some(NaiveDate::parse_from_str("2022-11-10", "%Y-%m-%d").unwrap()),
                ).await;
                if data.is_err() {
                    println!("error: {:?}", data);
                }
                assert!(data.is_ok());
                let data = data.unwrap();

                println!("data={:?}", data);
            })
    }

    #[test]
    fn test_fetch_fund_bar() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = HiqFundFetch::new();

                let data = fetch.fetch_fund_bar(
                    &to_std_code(MarketType::Fund, "159915"),
                    None,
                    None,
                    Some(NaiveDate::parse_from_str("2022-11-07", "%Y-%m-%d").unwrap()),
                    Some(NaiveDate::parse_from_str("2022-11-10", "%Y-%m-%d").unwrap()),
                ).await;
                if data.is_err() {
                    println!("error: {:?}", data);
                }
                assert!(data.is_ok());
                let data = data.unwrap();

                println!("data={:?}", data);
            })
    }
}
