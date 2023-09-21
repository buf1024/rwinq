use crate::comm::{async_client, fetch_bar, to_bar_ds};
use crate::stock::trans_info::{
    EastStockHotRankResult, EastStockIndex, EastStockIndustry, EastStockInfoMargin,
    EastStockMargin, EastStockYJBB, ExchSHStockInfo,
};
use crate::util::to_std_code;
use crate::{fetch_trade_date, Error, Market, MarketType, Result, HTTP_CMM_HEADER};
use calamine::{open_workbook_auto_from_rs, DataType, Reader};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::header::*;
use reqwest::Client;
use rwqcmm::{
    BarFreq, StockBar, StockComment, StockConcept, StockConceptBar, StockConceptDetail,
    StockHotRank, StockIndex, StockIndustry, StockIndustryBar, StockIndustryDetail, StockInfo,
    StockMargin, StockRtQuot, StockYJBB,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Cursor;
use std::ops::Add;

use super::trans_info::{
    EastStockComment, EastStockCommentAttention, EastStockCommentScore, EastStockQuot,
    ExchBJStockInfo,
};

pub struct StockFetch {
    client: Client,
}

impl StockFetch {
    pub fn new() -> Self {
        Self {
            client: async_client(),
        }
    }
    fn get_cell(&self, cell: &DataType) -> String {
        match cell {
            DataType::Int(cont) => {
                format!("{:0>6}", cont)
            }
            DataType::Float(cont) => {
                format!("{:0>6}", *cont as i64)
            }
            DataType::String(cont) => cont.clone(),
            _ => "".into(),
        }
    }
    /// 股票大盘指数（列举几个而已）
    pub async fn fetch_index_info(&self) -> Result<Vec<StockInfo>> {
        let data = vec![
            ("sh000001", "上证综指"),
            ("sz399001", "深证成指"),
            ("sz399006", "创业板指"),
            ("sz399102", "创业板综指"),
            ("sz399005", "中小板指"),
            ("sh000300", "沪深300"),
            ("sh000688", "科创50"),
            ("sz399673", "创业板50"),
            ("sz399550", "央视50"),
            ("sz399678", "深次新股"),
            ("sz399007", "深证300"),
            ("sz399008", "中小300"),
            ("bj899050", "北证50"),
        ];
        Ok(data
            .into_iter()
            .map(|(code, name)| StockInfo {
                code: code.to_owned(),
                name: name.to_owned(),
                block: "指数".to_owned(),
                is_margin: false,
                listing_date: Default::default(),
            })
            .collect())
    }
    /// 指数k线数据
    pub async fn fetch_index_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> Result<StockBar> {
        self.fetch_stock_bar(code, name, freq, start, end, skip_rt)
            .await
    }

    async fn fetch_stock_info_sh(&self, margin_codes: &HashSet<String>) -> Result<Vec<StockInfo>> {
        let mut data = Vec::new();
        let mut header = HTTP_CMM_HEADER.clone();
        // 上海
        header.insert(HOST, HeaderValue::from_static("query.sse.com.cn"));
        header.insert(
            REFERER,
            HeaderValue::from_static("http://www.sse.com.cn/assortment/stock/list/share/"),
        );
        let mut block_map = HashMap::new();
        block_map.insert("1", "主板");
        block_map.insert("8", "科创板");
        for (_, (stock_type, block_name)) in block_map.into_iter().enumerate() {
            // "主板A股": "1", "主板B股": "2", "科创板": "8"
            let req_url = format!(
                "http://query.sse.com.cn/sseQuery/commonQuery.do?\
    STOCK_TYPE={stock_type}&REG_PROVINCE=&CSRC_CODE=&STOCK_CODE=&\
    sqlId=COMMON_SSE_CP_GPJCTPZ_GPLB_GP_L&COMPANY_STATUS=2%2C4%2C5%2C7%2C8&type=inParams&\
    isPagination=true&pageHelp.cacheSize=1&\
    pageHelp.beginPage=1&pageHelp.pageSize=10000&pageHelp.pageNo=1&pageHelp.endPage=1&\
    _=1653291270045",
                stock_type = stock_type
            );

            let resp = self
                .client
                .get(req_url)
                .headers(header.clone())
                .send()
                .await?
                .text()
                .await?;

            let json: ExchSHStockInfo = serde_json::from_str(&resp)?;
            let tmp_vec: Vec<_> = json
                .page_help
                .data
                .iter()
                .filter(|item| item.de_list == "-")
                .map(|item| {
                    let code = to_std_code(MarketType::Stock, item.code);
                    let listing_date = NaiveDate::parse_from_str(item.list_date, "%Y%m%d").unwrap();
                    let listing_date =
                        NaiveDateTime::new(listing_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                    StockInfo {
                        code: code.clone(),
                        name: item.name.to_owned(),
                        block: block_name.to_owned(),
                        is_margin: margin_codes.contains(&code),
                        listing_date,
                    }
                })
                .collect();
            data.extend(tmp_vec.into_iter());
        }
        Ok(data)
    }

    async fn fetch_stock_info_sz(&self, margin_codes: &HashSet<String>) -> Result<Vec<StockInfo>> {
        let mut data = Vec::new();

        // "A股列表": "tab1", "B股列表": "tab2", "CDR列表": "tab3", "AB股列表": "tab4",
        let req_url = "http://www.szse.cn/api/report/ShowReport?SHOWTYPE=xlsx&CATALOGID=1110&\
         TABKEY=tab1&random=0.6935816432433362";

        let resp = self.client.get(req_url).send().await?.bytes().await?;
        // 板块	公司全称	英文名称	注册地址	A股代码	A股简称	A股上市日期	A股总股本	A股流通股本	B股代码
        // 	B股简称	B股上市日期	B股总股本	B股流通股本	地 区	省    份	城     市	所属行业	公司网址
        // 2712
        let mut workbook = open_workbook_auto_from_rs(Cursor::new(&*resp)).map_err(|e| {
            Error::Custom(format!(
                "Open shenzhen stock info xlsx stream error: {}!",
                e.to_string()
            ))
        })?;

        if let Some(Ok(range)) = workbook.worksheet_range("A股列表") {
            let tmp_vec: Vec<_> = range
                .rows()
                .skip(1) // 忽略表头
                .filter(|row| {
                    let code = self.get_cell(&row[4]);
                    if code.is_empty() {
                        return false;
                    }
                    code.chars().nth(0).unwrap().is_digit(10)
                })
                .map(|row| {
                    // 0 板块 4 A股代码 5 A股简称 6 A股上市日期
                    let code = to_std_code(MarketType::Stock, &self.get_cell(&row[4]));
                    let listing_date =
                        NaiveDate::parse_from_str(&self.get_cell(&row[6]), "%Y-%m-%d").unwrap();
                    let listing_date =
                        NaiveDateTime::new(listing_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                    StockInfo {
                        code: code.clone(),
                        name: self.get_cell(&row[5]),
                        block: self.get_cell(&row[0]),
                        is_margin: margin_codes.contains(&code),
                        listing_date,
                    }
                })
                .collect();
            data.extend(tmp_vec.into_iter());
        }

        Ok(data)
    }

    async fn fetch_stock_info_bj(&self, margin_codes: &HashSet<String>) -> Result<Vec<StockInfo>> {
        let mut data = Vec::new();

        let mut page = 0;
        let mut total_page = 0;
        let mut payload = HashMap::new();

        payload.insert("typejb", "T".to_owned());
        payload.insert("xxfcbj[]", "2".to_owned());
        payload.insert("xxzqdm", "".to_owned());
        payload.insert("pasortfieldge", "xxzqdm".to_owned());
        payload.insert("pasortfieldge", "asc".to_owned());

        let req_url = "https://www.bse.cn/nqxxController/nqxxCnzq.do";

        loop {
            payload.insert("page", page.to_string());
            let resp = self
                .client
                .post(req_url)
                .form(&payload)
                .send()
                .await?
                .text()
                .await?;

            let start = resp.find("[").unwrap();
            let end = resp.rfind("]").unwrap();

            let resp = &resp[start..=end];

            let json: Vec<ExchBJStockInfo> = serde_json::from_str(resp)?;
            json.into_iter().for_each(|info| {
                if total_page == 0 {
                    total_page = info.total_page;
                }

                let v: Vec<_> = info
                    .content
                    .into_iter()
                    .map(|item| {
                        let code = to_std_code(MarketType::Stock, item.code);
                        let listing_date =
                            NaiveDate::parse_from_str(item.list_date, "%Y%m%d").unwrap();
                        let listing_date = NaiveDateTime::new(
                            listing_date,
                            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                        );
                        StockInfo {
                            code: code.clone(),
                            name: item.name.to_owned(),
                            block: "主板".to_owned(),
                            is_margin: margin_codes.contains(&code),
                            listing_date,
                        }
                    })
                    .collect();
                data.extend(v)
            });
            page += 1;
            if page >= total_page {
                break;
            }
        }
        Ok(data)
    }

    /// 获取股票基本信息
    pub async fn fetch_stock_info(&self, market: Option<Market>) -> Result<Vec<StockInfo>> {
        let margin_codes = self.fetch_stock_is_margin().await?;

        let data = if let Some(m) = market {
            let mut data = Vec::new();
            match m {
                Market::SZ => {
                    data.extend(self.fetch_stock_info_sz(&margin_codes).await?.into_iter())
                }
                Market::SH => {
                    data.extend(self.fetch_stock_info_sh(&margin_codes).await?.into_iter())
                }
                Market::BJ => {
                    data.extend(self.fetch_stock_info_bj(&margin_codes).await?.into_iter())
                }
            }
            data
        } else {
            let mut data = Vec::new();
            data.extend(self.fetch_stock_info_sh(&margin_codes).await?.into_iter());
            data.extend(self.fetch_stock_info_sz(&margin_codes).await?.into_iter());
            data.extend(self.fetch_stock_info_bj(&margin_codes).await?.into_iter());
            data
        };
        Ok(data)
    }
    /// 获取融资融券股票代码
    pub async fn fetch_stock_is_margin(&self) -> Result<HashSet<String>> {
        let mut page = 1;
        let page_size: i32 = 2000;
        let mut total = 0;
        let mut data = HashSet::new();
        loop {
            let req_url = format!(
                "https://push2.eastmoney.com/api/qt/clist/get?\
            cb=jQuery1123017621166317571624_1639204790874&fid=f62&po=1&pz={page_size}&pn={page}&\
            np=1&fltt=2&invt=2&ut=b2884a393a59ad64002292a3e90d46a5&fs=b%3ABK0596&fields=f12",
                page = page,
                page_size = page_size
            );

            let resp = self.client.get(req_url).send().await?.text().await?;

            let js_text = &resp[43..resp.len() - 2];
            let json: EastStockInfoMargin = serde_json::from_str(js_text)?;

            let tmp_vec: HashSet<_> = json
                .data
                .diff
                .iter()
                .map(|item| to_std_code(MarketType::Stock, item.code))
                .collect();

            data.extend(tmp_vec.into_iter());

            if total == 0 {
                total = json.data.total;
            }
            if data.len() >= total {
                break;
            }
            page += 1;
        }
        Ok(data)
    }
    /// 股票k线数据
    pub async fn fetch_stock_bar(
        &self,
        code: &str,
        name: Option<&str>,
        freq: Option<BarFreq>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> Result<StockBar> {
        let market_code = if code.starts_with("sh") {
            // 上海市场
            format!("{}.{}", Market::SH as i32, &code[2..])
        } else {
            // 深圳和北京一样是0
            format!("{}.{}", Market::SZ as i32, &code[2..])
        };

        let freq = if freq.is_none() {
            BarFreq::Daily
        } else {
            freq.unwrap()
        };

        let bars = fetch_bar(&self.client, &market_code, code, freq, start, end, skip_rt).await?;
        let (stock_name, bars) = to_bar_ds(name, bars);
        let stock_bar = StockBar {
            code: code.to_owned(),
            name: stock_name,
            freq,
            bars,
        };
        Ok(stock_bar)
    }

    /// 股票最新指标
    pub async fn fetch_stock_index(
        &self,
        date: Option<NaiveDate>,
    ) -> Result<HashMap<String, StockIndex>> {
        let index_date = if let Some(d) = date {
            NaiveDateTime::new(d, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        } else {
            let trade_date = fetch_trade_date().await?;
            let mut n = Local::now().naive_local().date();
            let mut n_i32 = n.year() as i32 * 10000 + n.month() as i32 * 100 + n.day() as i32;
            while !trade_date.contains(&n_i32) {
                n = n.add(Duration::days(-1));
                n_i32 = n.year() as i32 * 10000 + n.month() as i32 * 100 + n.day() as i32;
            }
            NaiveDateTime::new(n, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        };

        let mut data = HashMap::new();
        let mut page_num: i64 = 1;
        let mut total: usize = 0;
        let page_size: usize = 2000;
        loop {
            let req_url = format!("https://push2.eastmoney.com/api/qt/clist/get?\
            pn={page_num}&pz={page_size}&po=1&np=1&ut=bd1d9ddb04089700cf9c27f6f7426281&fltt=2&invt=2&\
            fid=f3&fs=m:0+t:6,m:0+t:13,m:0+t:80,m:1+t:2,m:1+t:23&fields=f2,f9,f12,f14,f20,f21,f23&\
            _=1626075887768", page_num = page_num, page_size = page_size);

            let resp = self.client.get(req_url).send().await?.text().await?;

            let json = serde_json::from_str::<EastStockIndex>(&resp)?;
            if json.data.is_none() {
                break;
            }
            let js_data = json.data.unwrap();
            if total == 0 {
                total = js_data.total;
            }
            let tmp_vec: Vec<_> = js_data
                .diff
                .iter()
                .map(|item| {
                    let code = to_std_code(MarketType::Stock, item.code);
                    (
                        code.clone(),
                        StockIndex {
                            code,
                            name: item.name.to_owned(),
                            trade_date: index_date,
                            price: item.price.unwrap(),
                            pe: item.pe.unwrap(),
                            pb: item.pb.unwrap(),
                            total_value: item.total_value.unwrap(),
                            currency_value: item.currency_value.unwrap(),
                        },
                    )
                })
                .collect();

            data.extend(tmp_vec.into_iter());

            if data.len() >= total {
                break;
            }
            page_num += 1;
        }

        Ok(data)
    }

    /// 股票行业
    pub async fn fetch_stock_industry(&self) -> Result<Vec<StockIndustry>> {
        let req_url = format!(
            "http://push2.eastmoney.com/api/qt/clist/get?\
            pn=1&pz=2000&po=1&np=1&ut=bd1d9ddb04089700cf9c27f6f7426281&fltt=2&invt=2&fid=f3&\
            fs=m%3A90+t%3A2+f%3A%2150&fields=f1%2Cf2%2Cf3%2Cf4%2Cf5%2Cf6%2Cf7%2Cf8%2Cf9%2Cf10%2Cf12\
            %2Cf13%2Cf14%2Cf15%2Cf16%2Cf17%2Cf18%2Cf20%2Cf21%2Cf23%2Cf24%2Cf25%2Cf26%2Cf22%2Cf33\
            %2Cf11%2Cf62%2Cf128%2Cf136%2Cf115%2Cf152%2Cf124%2Cf107%2Cf104%2Cf105%2Cf140%2Cf141\
            %2Cf207%2Cf208%2Cf209%2Cf222&_=1626075887768"
        );

        let resp = self.client.get(req_url).send().await?.text().await?;

        let json: EastStockIndustry = serde_json::from_str(&resp)?;

        let data: Vec<_> = json
            .data
            .diff
            .iter()
            .map(|item| StockIndustry {
                code: item.code.to_owned(),
                name: item.name.to_owned(),
            })
            .collect();

        Ok(data)
    }
    /// 股票行业详情
    pub async fn fetch_stock_industry_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> Result<Vec<StockIndustryDetail>> {
        let codes = if let Some(c) = code {
            let n = if name.is_some() {
                name.unwrap().to_owned()
            } else {
                "".to_owned()
            };
            vec![StockIndustry {
                code: c.to_string(),
                name: n,
            }]
        } else {
            self.fetch_stock_industry().await?
        };

        let mut data = Vec::new();
        for industry in &codes {
            let req_url = format!(
                "http://push2.eastmoney.com/api/qt/clist/get?\
            pn=1&pz=2000&po=1&np=1&ut=bd1d9ddb04089700cf9c27f6f7426281&fltt=2&invt=2&fid=f3&\
            fs=b%3A{code}+f%3A%2150&fields=f1%2Cf2%2Cf3%2Cf4%2Cf5%2Cf6%2Cf7%2Cf8%2Cf9%2Cf10%2Cf12\
            %2Cf13%2Cf14%2Cf15%2Cf16%2Cf17%2Cf18%2Cf20%2Cf21%2Cf23%2Cf24%2Cf25%2Cf22%2Cf11%2Cf62\
            %2Cf128%2Cf136%2Cf115%2Cf152%2Cf45&_=1626081702127",
                code = &industry.code
            );

            let resp = self.client.get(req_url).send().await?.text().await?;

            let json: EastStockIndustry = serde_json::from_str(&resp)?;

            let stocks: Vec<_> = json
                .data
                .diff
                .iter()
                .map(|item| StockIndustryDetail {
                    code: industry.code.clone(),
                    name: industry.name.clone(),
                    stock_code: to_std_code(MarketType::Stock, item.code),
                    stock_name: item.name.to_owned(),
                })
                .collect();

            data.extend(stocks.into_iter());
        }

        Ok(data)
    }
    /// 股票行业k线数据
    pub async fn fetch_stock_industry_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> Result<StockIndustryBar> {
        let market_code = format!("90.{}", code);
        let freq = BarFreq::Daily;

        let bars = fetch_bar(&self.client, &market_code, code, freq, start, end, skip_rt).await?;
        let (stock_name, bars) = to_bar_ds(name, bars);
        let industry_bar = StockIndustryBar {
            code: code.to_owned(),
            name: stock_name,
            freq,
            bars,
        };
        Ok(industry_bar)
    }

    /// 股票概念
    pub async fn fetch_stock_concept(&self) -> Result<Vec<StockConcept>> {
        let req_url = format!(
            "http://push2.eastmoney.com/api/qt/clist/get?\
        pn=1&pz=2000&po=1&np=1&ut=bd1d9ddb04089700cf9c27f6f7426281&fltt=2&invt=2&fid=f3&\
        fs=m%3A90+t%3A3+f%3A%2150&fields=f2%2Cf3%2Cf4%2Cf8%2Cf12%2Cf14%2Cf15%2Cf16%2Cf17%2Cf18\
        %2Cf20%2Cf21%2Cf24%2Cf25%2Cf22%2Cf33%2Cf11%2Cf62%2Cf128%2Cf124%2Cf107%2Cf104%2Cf105%2Cf136\
        &_=1626075887768"
        );

        let resp = self.client.get(req_url).send().await?.text().await?;

        let json: EastStockIndustry = serde_json::from_str(&resp)?;

        let data: Vec<_> = json
            .data
            .diff
            .iter()
            .map(|item| StockConcept {
                code: item.code.to_owned(),
                name: item.name.to_owned(),
            })
            .collect();

        Ok(data)
    }

    /// 股票概念详情
    pub async fn fetch_stock_concept_detail(
        &self,
        code: Option<&str>,
        name: Option<&str>,
    ) -> Result<Vec<StockConceptDetail>> {
        let codes = if let Some(c) = code {
            let n = if name.is_some() {
                name.unwrap().to_owned()
            } else {
                "".to_owned()
            };
            vec![StockConcept {
                code: c.to_string(),
                name: n,
            }]
        } else {
            self.fetch_stock_concept().await?
        };

        let mut data = Vec::new();
        for concept in &codes {
            let req_url = format!(
                "http://push2.eastmoney.com/api/qt/clist/get?\
            pn=1&pz=2000&po=1&np=1&ut=bd1d9ddb04089700cf9c27f6f7426281&fltt=2&invt=2&fid=f3&\
            fs=b%3A{code}+f%3A%2150&fields=f1%2Cf2%2Cf3%2Cf4%2Cf5%2Cf6%2Cf7%2Cf8%2Cf9%2Cf10%2Cf12\
            %2Cf13%2Cf14%2Cf15%2Cf16%2Cf17%2Cf18%2Cf20%2Cf21%2Cf23%2Cf24%2Cf25%2Cf22%2Cf11%2Cf62\
            %2Cf128%2Cf136%2Cf115%2Cf152%2Cf45&_=1626081702127",
                code = &concept.code
            );

            let resp = self.client.get(req_url).send().await?.text().await?;

            let json: EastStockIndustry = serde_json::from_str(&resp)?;

            let stocks: Vec<_> = json
                .data
                .diff
                .iter()
                .map(|item| StockConceptDetail {
                    code: concept.code.clone(),
                    name: concept.name.clone(),
                    stock_code: to_std_code(MarketType::Stock, item.code),
                    stock_name: item.name.to_owned(),
                })
                .collect();

            data.extend(stocks.into_iter());
        }

        Ok(data)
    }

    /// 股票概念k线数据
    pub async fn fetch_stock_concept_daily(
        &self,
        code: &str,
        name: Option<&str>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        skip_rt: bool,
    ) -> Result<StockConceptBar> {
        let market_code = format!("90.{}", code);
        let freq = BarFreq::Daily;

        let bars = fetch_bar(&self.client, &market_code, code, freq, start, end, skip_rt).await?;
        let (stock_name, bars) = to_bar_ds(name, bars);
        let industry_bar = StockConceptBar {
            code: code.to_owned(),
            name: stock_name,
            freq,
            bars,
        };
        Ok(industry_bar)
    }

    /// 股票业绩报表
    pub async fn fetch_stock_yjbb(&self, year: u16, season: u16) -> Result<Vec<StockYJBB>> {
        let season_vec = vec!["03-31", "06-30", "09-30", "12-31"];

        if year < 1991 || year > 2050 {
            return Err(Error::Custom(format!("Invalid year: {}", year)));
        }
        if season < 1 || season > 4 {
            return Err(Error::Custom(format!("Invalid season: {}", season)));
        }
        let season_date = format!(
            "{}-{}",
            year,
            *season_vec.get((season - 1) as usize).unwrap()
        );

        let mut page = 1;
        let page_size: i32 = 500;
        let mut data = Vec::new();
        let mut total_page = 0;

        loop {
            let req_url = format!("http://datacenter.eastmoney.com/api/data/get?\
            st=UPDATE_DATE%2CSECURITY_CODE&sr=-1%2C-1&ps={page_size}&p={page}&type=RPT_LICO_FN_CPD&sty=ALL&\
            token=894050c76af8597a853f5b408b759f5d&filter=%28REPORTDATE%3D%27{season_date}%27%29",
                                  page_size = page_size, page = page, season_date = season_date);

            let resp = self.client.get(req_url).send().await?.text().await?;

            let json: EastStockYJBB = serde_json::from_str(&resp)?;

            if json.result.is_none() {
                break;
            }
            let result = json.result.unwrap();
            let tmp_vec: Vec<_> = result
                .data
                .iter()
                .map(|item| {
                    let season_date =
                        NaiveDateTime::parse_from_str(item.season_date, "%Y-%m-%d %H:%M:%S")
                            .unwrap();
                    StockYJBB {
                        year,
                        season,
                        season_date,
                        code: to_std_code(MarketType::Stock, item.code),
                        name: item.name.to_owned(),
                        mg_sy: item.mg_sy.unwrap_or_default(),
                        yysr: item.yysr.unwrap_or_default(),
                        yysr_tbzz: item.yysr_tbzz.unwrap_or_default(),
                        yysr_jdhbzz: item.yysr_jdhbzz.unwrap_or_default(),
                        jlr: item.jlr.unwrap_or_default(),
                        jlr_tbzz: item.jlr_tbzz.unwrap_or_default(),
                        jlr_jdhbzz: item.jlr_jdhbzz.unwrap_or_default(),
                        mg_jzc: item.mg_jzc.unwrap_or_default(),
                        jzc_syl: item.mg_jzc.unwrap_or_default(),
                        mg_jy_xjl: item.mg_jy_xjl.unwrap_or_default(),
                        xs_mll: item.xs_mll.unwrap_or_default(),
                    }
                })
                .collect();

            data.extend(tmp_vec.into_iter());

            if total_page == 0 {
                total_page = result.pages;
            }
            if page == total_page {
                break;
            }
            page += 1;
        }
        Ok(data)
    }
    /// 融资融券
    pub async fn fetch_stock_margin(
        &self,
        code: &str,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<Vec<StockMargin>> {
        let mut page = 1;
        let page_size: i32 = 500;
        let mut data = Vec::new();
        let mut total_page = 0;

        let s = start.unwrap_or(NaiveDate::parse_from_str("19910101", "%Y%m%d").unwrap());

        let e = end.unwrap_or(Local::now().date_naive());

        loop {
            let req_url = format!(
                "http://datacenter-web.eastmoney.com/api/data/v1/get?\
            reportName=RPTA_WEB_RZRQ_GGMX&columns=ALL&source=WEB&sortColumns=date&sortTypes=-1&\
            pageNumber={page}&pageSize={page_size}&filter=(scode%3D%22{code}%22)&pageNo={page}&\
            _=1668232304568",
                page_size = page_size,
                page = page,
                code = &code[2..]
            );

            let resp = self.client.get(req_url).send().await?.text().await?;

            let json = serde_json::from_str::<EastStockMargin>(&resp)?;

            if json.result.is_none() {
                break;
            }
            let result = json.result.unwrap();
            let tmp_vec: Vec<_> = result
                .data
                .iter()
                .map(|item| StockMargin {
                    code: to_std_code(MarketType::Stock, item.code),
                    name: item.name.to_owned(),
                    trade_date: NaiveDateTime::parse_from_str(item.trade_date, "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                    close: item.close.unwrap_or(0.0),
                    chg_pct: item.chg_pct.unwrap_or(0.0),
                    rz_ye: item.rq_ye.unwrap_or(0.0),
                    rz_ye_zb: item.rz_ye_zb.unwrap_or(0.0),
                    rz_mre: item.rz_mre.unwrap_or(0.0),
                    rz_che: item.rz_che.unwrap_or(0.0),
                    rz_jme: item.rz_jme.unwrap_or(0.0),
                    rq_ye: item.rq_ye.unwrap_or(0.0),
                    rq_yl: item.rq_yl.unwrap_or(0),
                    rq_mcl: item.rq_mcl.unwrap_or(0),
                    rq_chl: item.rq_chl.unwrap_or(0),
                    rq_jmg: item.rq_jmg.unwrap_or(0),
                    rz_rq_ye: item.rz_rq_ye.unwrap_or(0.0),
                    rz_rq_ye_cz: item.rz_rq_ye_cz.unwrap_or(0.0),
                })
                .filter(|item| item.trade_date.date() >= s && item.trade_date.date() <= e)
                .collect();
            if tmp_vec.len() > 0 {
                let (newest, oldest) = (&tmp_vec[0], &tmp_vec[tmp_vec.len() - 1]);
                let is_break = if newest.trade_date.date() >= e && oldest.trade_date.date() <= s {
                    true
                } else {
                    false
                };
                data.extend(tmp_vec.into_iter());
                if is_break {
                    break;
                }
            }

            if total_page == 0 {
                total_page = result.pages;
            }
            if page == total_page {
                break;
            }

            page += 1;
        }
        Ok(data)
    }

    /// 股票排名
    pub async fn fetch_stock_hot_rank(&self, code: &str) -> Result<StockHotRank> {
        let req_url = "https://emappdata.eastmoney.com/stockrank/getCurrentLatest";

        let mut map = HashMap::new();
        map.insert("appId", "appId01");
        map.insert("globalId", "786e4c21-70dc-435a-93bb-38");
        map.insert("srcSecurityCode", code);

        let resp = self
            .client
            .post(req_url)
            .json(&map)
            .send()
            .await?
            .text()
            .await?;

        let json: EastStockHotRankResult = serde_json::from_str(&resp)?;
        let data = json.data;

        Ok(StockHotRank {
            code: code.into(),
            market_all_count: data.market_all_count,
            rank: data.rank,
            rank_chg: data.rank_chg,
            calc_time: NaiveDateTime::parse_from_str(data.calc_time, "%Y-%m-%d %H:%M:%S").unwrap(),
        })
    }

    /// 全量股票行情，注意调用频率
    pub async fn fetch_stock_rt_quot(
        &self,
        codes: Option<Vec<String>>,
    ) -> Result<Vec<StockRtQuot>> {
        let req_url = format!(
            "http://82.push2.eastmoney.com/api/qt/clist/get?pn=1&pz=50000&po=1&np=1&ut=bd1d9ddb04089700cf9c27f6f7426281&fltt=2&invt=2&fid=f3&fs=m%3A0+t%3A6%2Cm%3A0+t%3A80%2Cm%3A1+t%3A2%2Cm%3A1+t%3A23%2Cm%3A0+t%3A81+s%3A2048&fields=f1%2Cf2%2Cf3%2Cf4%2Cf5%2Cf6%2Cf7%2Cf8%2Cf9%2Cf10%2Cf12%2Cf13%2Cf14%2Cf15%2Cf16%2Cf17%2Cf18%2Cf20%2Cf21%2Cf23%2Cf24%2Cf25%2Cf22%2Cf11%2Cf62%2Cf128%2Cf136%2Cf115%2Cf152&_=1623833739532"
        );

        let resp = self.client.get(req_url).send().await?.text().await?;

        let json: EastStockQuot = serde_json::from_str(&resp)?;

        let codes: Option<Vec<String>> =
            codes.map(|codes| codes.iter().map(|code| String::from(&code[2..])).collect());
        let data: Vec<_> = json.data.map_or_else(
            || Vec::new(),
            |data| {
                data.diff
                    .iter()
                    .filter(|item| {
                        item.open.unwrap() > 0.0
                            && (codes.is_none()
                                || codes.as_ref().unwrap().contains(&String::from(item.code)))
                    })
                    .map(|item| StockRtQuot {
                        code: to_std_code(MarketType::Stock, item.code),
                        name: item.name.to_owned(),
                        price: item.price.unwrap(),
                        chg_pct: item.chg_pct.unwrap(),
                        chg: item.chg.unwrap(),
                        volume: item.volume.unwrap(),
                        amount: item.amount.unwrap(),
                        turnover: item.turnover.unwrap(),
                        pe: item.pe.unwrap(),
                        vol_ratio: item.vol_ratio.unwrap(),
                        high: item.high.unwrap(),
                        low: item.low.unwrap(),
                        open: item.open.unwrap(),
                        last_close: item.last_close.unwrap(),
                        total_value: item.total_value.unwrap(),
                        currency_value: item.currency_value.unwrap(),
                        rise_speed: item.rise_speed.unwrap(),
                        pb: item.pb.clone().unwrap_or_default().unwrap(),
                    })
                    .collect()
            },
        );

        Ok(data)
    }
    /// 千股千评
    pub async fn fetch_stock_comment(
        &self,
        codes: Option<Vec<String>>,
    ) -> Result<Vec<StockComment>> {
        let mut page = 1;
        let page_size: i32 = 500;
        let mut data = Vec::new();
        let mut total_page = 0;

        let codes: Option<Vec<String>> =
            codes.map(|codes| codes.iter().map(|code| String::from(&code[2..])).collect());

        loop {
            let req_url = format!(
                "https://datacenter-web.eastmoney.com/api/data/v1/get?\
                sortColumns=SECURITY_CODE&sortTypes=1&pageSize={page_size}0&pageNumber={page}&reportName=RPT_DMSK_TS_STOCKNEW&quoteColumns=f2~01~SECURITY_CODE~CLOSE_PRICE%2Cf8~01~SECURITY_CODE~TURNOVERRATE%2Cf3~01~SECURITY_CODE~CHANGE_RATE%2Cf9~01~SECURITY_CODE~PE_DYNAMIC&columns=ALL&filter=&token=894050c76af8597a853f5b408b759f5d",
                page_size = page_size,
                page = page,
            );

            let resp = self.client.get(req_url).send().await?.text().await?;

            let json: EastStockComment = serde_json::from_str(&resp)?;

            if json.result.is_none() {
                break;
            }
            let result = json.result.unwrap();
            let tmp_vec: Vec<_> = result
                .data
                .iter()
                .filter(|item| {
                    codes.is_none() || codes.as_ref().unwrap().contains(&String::from(item.code))
                })
                .map(|item| {
                    let name = item.name.to_owned();
                    let mut name_vec = Vec::new();
                    for n in name.chars() {
                        if !n.is_whitespace() {
                            name_vec.push(n)
                        }
                    }
                    let name: String = name_vec.into_iter().collect();
                    StockComment {
                        code: to_std_code(MarketType::Stock, item.code),
                        name,
                        trade_date: NaiveDateTime::parse_from_str(
                            item.trade_date,
                            "%Y-%m-%d %H:%M:%S",
                        )
                        .unwrap(),
                        close: item.close.unwrap(),
                        chg_pct: item.chg_pct.unwrap(),
                        turnover: item.turnover,
                        pe: item.pe,
                        cost: item.cost,
                        engage: item.engage,
                        score: item.score.unwrap_or_default(),
                        rank: item.rank.unwrap_or_default(),
                        rank_chg: item.rank_chg.unwrap_or_default(),
                        attention: item.attention.unwrap_or_default(),
                    }
                })
                .collect();
            if tmp_vec.len() > 0 {
                data.extend(tmp_vec.into_iter());
            }

            if total_page == 0 {
                total_page = result.pages;
            }
            if page == total_page {
                break;
            }

            page += 1;
        }
        Ok(data)
    }

    // async fetch_attention_rank_data(&self, code: String) ->
    /// 千股千评历史
    pub async fn fetch_stock_comment_his(&self, code: String) -> Result<Vec<StockComment>> {
        // 机构参与度 / 市场成本

        let req_url = format!(
            "https://datacenter-web.eastmoney.com/api/data/v1/get?\
            reportName=RPT_DMSK_TS_STOCKEVALUATE&filter=%28SECURITY_CODE%3D%22{code}%22%29&\
            columns=ALL&source=WEB&client=WEB&sortColumns=TRADE_DATE&sortTypes=-1&_=1655387358195",
            code = &code[2..]
        );

        let resp = self.client.get(req_url).send().await?.text().await?;

        let json: EastStockComment = serde_json::from_str(&resp)?;

        if json.result.is_none() {
            return Ok(Vec::new());
        }
        let engage_cost_data: BTreeMap<_, _> = json
            .result
            .unwrap()
            .data
            .into_iter()
            .map(|item| (item.trade_date, item))
            .collect();

        // 市场评分

        let req_url = format!(
            "https://datacenter-web.eastmoney.com/api/data/v1/get?\
            filter=(SECURITY_CODE%3D%22{code}%22)&columns=ALL&source=WEB&client=WEB&\
            reportName=RPT_STOCK_HISTORYMARK&sortColumns=DIAGNOSE_DATE&sortTypes=1&_=1695281367385",
            code = &code[2..]
        );

        let resp = self.client.get(req_url).send().await?.text().await?;

        let json: EastStockCommentScore = serde_json::from_str(&resp)?;

        if json.result.is_none() {
            return Ok(Vec::new());
        }
        let score_data: HashMap<_, _> = json
            .result
            .unwrap()
            .data
            .into_iter()
            .map(|item| (item.trade_date, item))
            .collect();

        // 关注度 / 排名

        let req_url = format!(
            "https://datacenter-web.eastmoney.com/api/data/v1/get?\
            filter=(SECURITY_CODE%3D%22{code}%22)&columns=ALL&source=WEB&client=WEB&\
            reportName=RPT_STOCK_MARKETFOCUS&sortColumns=TRADE_DATE&sortTypes=-1&pageSize=30&_=1695281367390",
            code = &code[2..]);

        let resp = self.client.get(req_url).send().await?.text().await?;

        let json: EastStockCommentAttention = serde_json::from_str(&resp)?;

        if json.result.is_none() {
            return Ok(Vec::new());
        }
        let attention_rank_data: HashMap<_, _> = json
            .result
            .unwrap()
            .data
            .into_iter()
            .map(|item| (item.trade_date, item))
            .collect();

        let data = engage_cost_data
            .into_iter()
            .map(|(trade_date, item)| {
                let name = item.name.to_owned();
                let mut name_vec = Vec::new();
                for n in name.chars() {
                    if !n.is_whitespace() {
                        name_vec.push(n)
                    }
                }
                let name: String = name_vec.into_iter().collect();

                let score = score_data
                    .get(trade_date)
                    .map_or(0.0, |s_item| s_item.score);

                let (rank, rank_chg, attention) =
                    attention_rank_data
                        .get(trade_date)
                        .map_or((0, 0, 0.0), |a_item| {
                            (
                                a_item.rank.unwrap_or_default(),
                                a_item.rank_chg.unwrap_or_default(),
                                a_item.attention,
                            )
                        });
                StockComment {
                    code: to_std_code(MarketType::Stock, item.code),
                    name,
                    trade_date: NaiveDateTime::parse_from_str(item.trade_date, "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                    close: item.close.unwrap(),
                    chg_pct: item.chg_pct.unwrap(),
                    turnover: item.turnover,
                    pe: item.pe,
                    cost: item.cost,
                    engage: item.engage,
                    score,
                    rank,
                    rank_chg,
                    attention,
                }
            })
            .collect();

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::StockFetch;
    use chrono::NaiveDate;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    fn setup() {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));
        // 输出到控制台中
        let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(ErrorLayer::default())
            .with(formatting_layer)
            .init();
    }

    #[test]
    fn test_fetch_stock_info() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_info(None).await;
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);

                let d1 = data
                    .iter()
                    .filter(|item| !item.is_margin && item.code.starts_with("sh"))
                    .nth(0);
                let d1 = d1.unwrap();
                println!("d1: {:?}", d1);

                let d2 = data
                    .iter()
                    .filter(|item| item.is_margin && item.code.starts_with("sh"))
                    .nth(0);
                let d2 = d2.unwrap();
                println!("d1: {:?}", d2);

                let d1 = data
                    .iter()
                    .filter(|item| !item.is_margin && item.code.starts_with("sz"))
                    .nth(0);
                let d1 = d1.unwrap();
                println!("d1: {:?}", d1);

                let d2 = data
                    .iter()
                    .filter(|item| {
                        item.block == "创业板" && item.is_margin && item.code.starts_with("sz")
                    })
                    .nth(0);
                let d2 = d2.unwrap();
                println!("d2: {:?}", d2);

                let d3 = data
                    .iter()
                    .filter(|item| item.is_margin && item.code.starts_with("bj"))
                    .nth(0);
                let d3 = d3.unwrap();
                println!("d3: {:?}", d3);
            })
    }

    #[test]
    fn test_fetch_stock_is_margin() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_is_margin().await;
                assert!(data.is_ok());

                let data = data.unwrap();
                assert!(data.len() > 0);

                let v: Vec<_> = data.iter().take(1).collect();
                println!("data={}", v[0]);
            })
    }

    #[test]
    fn fetch_stock_bar() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                setup();

                let start = NaiveDate::parse_from_str("20220301", "%Y%m%d").unwrap();
                // let end = start.clone();
                let fetch = StockFetch::new();
                let data = fetch
                    .fetch_stock_bar("sz000001", None, None, Some(start), None, false)
                    .await;

                assert!(data.is_ok());

                let data = data.unwrap();
                assert!(data.bars.is_some());

                let bars = data.bars.unwrap();
                assert!(bars.len() > 0);

                println!("bars[0]={:?}", bars[0]);
                println!("bars[-1]={:?}", bars[bars.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_index() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_index(None).await;
                assert!(data.is_ok());

                let data = data.unwrap();
                println!("data len={}", data.len());
                assert!(data.len() > 0);

                let v_data: Vec<_> = data.values().collect();
                println!("data[0]={:?}", v_data[0]);
                println!("data[-1]={:?}", v_data[v_data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_industry() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_industry().await;
                assert!(data.is_ok());

                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_industry_detail() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_industry_detail(None, None).await;
                assert!(data.is_ok());

                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_industry_daily() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch
                    .fetch_stock_industry_daily("BK1044", None, None, None, true)
                    .await;
                assert!(data.is_ok());
                let data = data.unwrap();
                let data = data.bars;
                assert!(data.is_some());
                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_concept() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_concept().await;
                assert!(data.is_ok());

                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_concept_detail() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_concept_detail(Some("BK1109"), None).await;
                assert!(data.is_ok());

                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);

                println!("data={:?}", data)
            })
    }

    #[test]
    fn test_fetch_stock_concept_daily() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch
                    .fetch_stock_concept_daily("BK0969", None, None, None, true)
                    .await;
                assert!(data.is_ok());
                let data = data.unwrap();
                let data = data.bars;
                assert!(data.is_some());
                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_yjbb() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let data = fetch.fetch_stock_yjbb(2022, 3).await;
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);

                println!("data[0]={:?}", data[0]);
                println!("data[-1]={:?}", data[data.len() - 1]);
            })
    }

    #[test]
    fn test_fetch_stock_margin() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();
                let start = NaiveDate::parse_from_str("2022-11-30", "%Y-%m-%d").unwrap();
                let end = NaiveDate::parse_from_str("2022-12-1", "%Y-%m-%d").unwrap();
                let data = fetch
                    .fetch_stock_margin("sh601928", Some(start), Some(end))
                    .await;
                assert!(data.is_ok());
                let data = data.unwrap();
                assert!(data.len() > 0);
                data.iter().enumerate().for_each(|(i, item)| {
                    println!("data[{}]={:?}", i, item);
                });
            })
    }

    #[test]
    fn test_fetch_stock_hot_rank() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();

                let data = fetch.fetch_stock_hot_rank("sz300468").await;
                assert!(data.is_ok());
                let data = data.unwrap();

                println!("data={:?}", data);
            })
    }
    #[test]
    fn test_fetch_stock_rt_quot() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();

                let data = fetch
                    .fetch_stock_rt_quot(Some(vec!["sz000001".into(), "sh603270".into()]))
                    .await;
                if data.is_err() {
                    println!("{:?}", data.as_ref().err().unwrap());
                }
                assert!(data.is_ok());
                let data = data.unwrap();
                println!("data len={:?}", data.len());
                println!("data={:?}", data[1]);
            })
    }
    #[test]
    fn test_fetch_stock_comment() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();

                let data = fetch.fetch_stock_comment(None).await;
                if data.is_err() {
                    println!("{:?}", data.as_ref().err().unwrap());
                }
                assert!(data.is_ok());
                let data = data.unwrap();
                println!("data len={:?}", data.len());
                println!("data={:?}", data[1]);
            })
    }
    #[test]
    fn test_fetch_stock_comment_his() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let fetch = StockFetch::new();

                let data = fetch.fetch_stock_comment_his("sh600000".into()).await;
                if data.is_err() {
                    println!("{:?}", data.as_ref().err().unwrap());
                }
                assert!(data.is_ok());
                let data = data.unwrap();
                println!("data len={:?}", data.len());
                println!("data={:?}", data[1]);
            })
    }
}
