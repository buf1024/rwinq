use crate::bond::hiq_bond_info::EastBondInfo;
use crate::bond::BondFetch;
use crate::comm::{async_client, fetch_bar};
use crate::util::to_std_code;
use crate::{Market, MarketType, Result, HTTP_CMM_HEADER};
use async_trait::async_trait;
use chrono::naive::NaiveDate;
use chrono::NaiveDateTime;
use hiq_common::{BarFreq, BondBar, BondInfo};
use reqwest::Client;

pub struct HiqBondFetch {
    client: Client,
}

impl HiqBondFetch {
    pub fn new() -> Self {
        Self {
            client: async_client(),
        }
    }
}

#[async_trait]
impl BondFetch for HiqBondFetch {
    /// 获取可转债基本信息
    async fn fetch_bond_info(&self) -> Result<Vec<BondInfo>> {
        let mut data = Vec::new();

        let mut page_num: i64 = 1;
        let mut pages: i64 = -1;
        loop {
            let req_url = format!(
                "https://datacenter-web.eastmoney.com/api/data/v1/get?\
            sortColumns=PUBLIC_START_DATE&sortTypes=-1&pageSize=500&pageNumber={page_num}&\
            reportName=RPT_BOND_CB_LIST&columns=ALL&\
            quoteColumns=f2~01~CONVERT_STOCK_CODE~CONVERT_STOCK_PRICE,\
            f235~10~SECURITY_CODE~TRANSFER_PRICE,f236~10~SECURITY_CODE~TRANSFER_VALUE,\
            f2~10~SECURITY_CODE~CURRENT_BOND_PRICE,f237~10~SECURITY_CODE~TRANSFER_PREMIUM_RATIO,\
            f239~10~SECURITY_CODE~RESALE_TRIG_PRICE,f240~10~SECURITY_CODE~REDEEM_TRIG_PRICE,\
            f23~01~CONVERT_STOCK_CODE~PBV_RATIO&source=WEB&client=WEB",
                page_num = page_num
            );

            let resp = self
                .client
                .get(req_url)
                .headers(HTTP_CMM_HEADER.to_owned())
                .send()
                .await?
                .text()
                .await?;

            let json: EastBondInfo = serde_json::from_str(&resp)?;

            if pages == -1 {
                pages = json.result.pages;
            }

            let tmp_vec: Vec<_> = json
                .result
                .data
                .iter()
                .filter(|f| f.listing_date.is_some() && f.delist_date.is_none())
                .map(|item| {
                    let listing_date = item.listing_date.unwrap();
                    let listing_date =
                        NaiveDateTime::parse_from_str(listing_date, "%Y-%m-%d %H:%M:%S").unwrap();
                    BondInfo {
                        code: to_std_code(MarketType::Bond, item.code),
                        name: item.name.to_owned(),
                        stock_code: to_std_code(MarketType::Stock, item.stock_code),
                        stock_name: item.stock_name.to_owned(),
                        listing_date,
                        is_delist: 0,
                    }
                })
                .collect();

            data.extend(tmp_vec.into_iter());

            page_num += 1;
            if page_num > pages {
                break;
            }
        }

        Ok(data)
    }

    /// 获取可转债基本
    ///
    /// *code* 可转债代码，其中11开头的为深市，12开头的为沪市。
    async fn fetch_bond_bar(
        &self,
        code: &str,
        name: &str,
        stock_code: &str,
        stock_name: &str,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> Result<BondBar> {
        let market_code = if code.starts_with("sz") {
            // 深圳市场
            format!("{}.{}", Market::SZ as i32, &code[2..])
        } else {
            format!("{}.{}", Market::SH as i32, &code[2..])
        };
        let freq = if freq.is_none() {
            BarFreq::Daily
        } else {
            freq.unwrap()
        };

        let bars = fetch_bar(&self.client, &market_code, code, freq, start, end, skip_rt).await?;
        let bond_bar = BondBar {
            code: code.to_owned(),
            name: name.to_owned(),
            stock_code: stock_code.to_owned(),
            stock_name: stock_name.to_owned(),
            freq,
            bars: if bars.len() > 0 { Some(bars) } else { None },
        };
        Ok(bond_bar)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bond_fetch, BondFetch};
    use hiq_common::BarFreq;
    // use chrono::NaiveDate;

    #[test]
    fn test_fetch_bond_info() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = bond_fetch();
                let res = fetch.fetch_bond_info().await.unwrap();

                assert!(res.len() > 0);
                println!("{:?}", res[0]);
            })
    }

    #[test]
    fn test_fetch_bond_bar() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = bond_fetch();
                let bond_info = fetch.fetch_bond_info().await.unwrap();

                let item = bond_info.get(bond_info.len() - 1).unwrap();

                let res = fetch
                    .fetch_bond_bar(
                        &item.code[..],
                        &item.name[..],
                        &item.stock_code[..],
                        &item.stock_name[..],
                        Some(BarFreq::Weekly),
                        None,
                        None,
                        true
                    )
                    .await
                    .unwrap();

                // let res = fetch.fetch_bond_bar("123114", "三角转债",
                //                                "sz300775", "三角防务",
                //                                Some(BarFreq::Daily),
                //                                Some(NaiveDate::parse_from_str("20221111", "%Y%m%d")
                //                                           .unwrap()),
                //                                None).unwrap();

                assert!(res.bars.is_some());
                let bar = res.bars.unwrap();
                println!("{}", bar.len());
                let first_bar = bar.get(0).unwrap();
                let last_bar = bar.get(bar.len() - 1).unwrap();
                println!("{:?}", first_bar);
                println!("{:?}", last_bar);
            })
    }
}
