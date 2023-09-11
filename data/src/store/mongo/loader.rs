use crate::{
    store::{
        Loader, TAB_BOND_DAILY, TAB_BOND_INFO, TAB_FUND_DAILY, TAB_FUND_INFO, TAB_FUND_NET,
        TAB_INDEX_DAILY, TAB_INDEX_INFO, TAB_STOCK_CONCEPT, TAB_STOCK_CONCEPT_DAILY,
        TAB_STOCK_CONCEPT_DETAIL, TAB_STOCK_DAILY, TAB_STOCK_INDEX, TAB_STOCK_INDUSTRY,
        TAB_STOCK_INDUSTRY_DAILY, TAB_STOCK_INDUSTRY_DETAIL, TAB_STOCK_INFO, TAB_STOCK_MARGIN,
        TAB_STOCK_YJBB,
    },
    Error, Result,
};
use async_trait::async_trait;
use mongodb::{
    bson::Document,
    options::{ClientOptions, FindOptions},
    Client,
};
use serde::de::DeserializeOwned;

use super::{query, query_one};

pub(crate) struct MongoLoader {
    client: Option<Client>,
    url: String,
}

impl MongoLoader {
    pub fn new(url: String) -> Self {
        Self { url, client: None }
    }
    async fn build_client(&mut self) -> Result<Client> {
        if self.client.is_some() {
            let client = self.client.as_ref().unwrap();
            return Ok(client.clone());
        }

        let mut client_options = ClientOptions::parse(&self.url[..]).await.map_err(|e| {
            log::error!("parse connect url error: {}", e.to_string());
            Error::Custom(format!("parse connect url error: {}", e.to_string()))
        })?;

        client_options.app_name = Some("HIQ App".to_string());

        let client = Client::with_options(client_options).map_err(|e| {
            log::error!("with_options error: {}", e.to_string());
            Error::Custom(format!("with_options error: {}", e.to_string()))
        })?;
        self.client = Some(client.clone());
        Ok(client)
    }
    fn get_client(&self) -> Result<Client> {
        let client = self
            .client
            .as_ref()
            .ok_or(Error::Custom("mongodb not connected!".to_owned()))?;
        Ok(client.clone())
    }
    async fn query<T>(
        &self,
        tab: &str,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Unpin + Send + Sync + Clone,
    {
        if let Some(limit) = limit {
            if limit == 1 {
                let data: Option<T> = self.query_one(tab, filter, sort).await?;
                return data.map_or(Ok(Vec::new()), |e| Ok(vec![e]));
            }
        }

        self.query_all(tab, filter, sort, limit).await
    }
    async fn query_all<T>(
        &self,
        tab: &str,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Unpin + Send + Sync + Clone,
    {
        let client = self.get_client()?;
        let options = if let Some(limit) = limit {
            FindOptions::builder().sort(sort).limit(limit).build()
        } else {
            FindOptions::builder().sort(sort).build()
        };
        let data: Vec<T> = query(client, tab, filter, options).await?;
        Ok(data)
    }
    async fn query_one<T>(&self, tab: &str, filter: Document, sort: Document) -> Result<Option<T>>
    where
        T: DeserializeOwned + Unpin + Send + Sync + Clone,
    {
        let client = self.get_client()?;
        let options = FindOptions::builder().sort(sort).build();

        let data: Option<T> = query_one(client, tab, filter, options).await?;
        Ok(data)
    }
}

#[async_trait]
impl Loader for MongoLoader {
    async fn init(&mut self) -> Result<()> {
        self.build_client().await?;
        Ok(())
    }
    async fn load_bond_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::BondInfo>> {
        self.query(TAB_BOND_INFO, filter, sort, limit).await
    }
    async fn load_bond_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::Bar>> {
        self.query(TAB_BOND_DAILY, filter, sort, limit).await
    }

    async fn load_fund_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::FundInfo>> {
        self.query(TAB_FUND_INFO, filter, sort, limit).await
    }
    async fn load_fund_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::Bar>> {
        self.query(TAB_FUND_DAILY, filter, sort, limit).await
    }
    async fn load_fund_net(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::FundNet>> {
        self.query(TAB_FUND_NET, filter, sort, limit).await
    }

    async fn load_index_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockInfo>> {
        self.query(TAB_INDEX_INFO, filter, sort, limit).await
    }

    async fn load_index_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::Bar>> {
        self.query(TAB_INDEX_DAILY, filter, sort, limit).await
    }

    async fn load_stock_info(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockInfo>> {
        self.query(TAB_STOCK_INFO, filter, sort, limit).await
    }

    async fn load_stock_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::Bar>> {
        self.query(TAB_STOCK_DAILY, filter, sort, limit).await
    }

    async fn load_stock_index(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockIndex>> {
        self.query(TAB_STOCK_INDEX, filter, sort, limit).await
    }
    async fn load_stock_industry(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockIndustry>> {
        self.query(TAB_STOCK_INDUSTRY, filter, sort, limit).await
    }

    async fn load_stock_industry_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::Bar>> {
        self.query(TAB_STOCK_INDUSTRY_DAILY, filter, sort, limit)
            .await
    }
    async fn load_stock_industry_detail(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockIndustryDetail>> {
        self.query(TAB_STOCK_INDUSTRY_DETAIL, filter, sort, limit)
            .await
    }

    async fn load_stock_concept(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockConcept>> {
        self.query(TAB_STOCK_CONCEPT, filter, sort, limit).await
    }

    async fn load_stock_concept_daily(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::Bar>> {
        self.query(TAB_STOCK_CONCEPT_DAILY, filter, sort, limit)
            .await
    }
    async fn load_stock_concept_detail(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockConceptDetail>> {
        self.query(TAB_STOCK_CONCEPT_DETAIL, filter, sort, limit)
            .await
    }

    async fn load_stock_yjbb(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockYJBB>> {
        self.query(TAB_STOCK_YJBB, filter, sort, limit).await
    }

    async fn load_stock_margin(
        &self,
        filter: Document,
        sort: Document,
        limit: Option<i64>,
    ) -> Result<Vec<rwqfetch::StockMargin>> {
        self.query(TAB_STOCK_MARGIN, filter, sort, limit).await
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;
    use mongodb::bson::doc;

    use crate::store::mongo::MongoLoader;
    use crate::store::Loader;

    #[test]
    fn test_loader_async() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let res = test_loader().await;
                if res.is_err() {
                    println!("Error: {:?}", res);
                }
            })
    }

    async fn test_loader() -> Result<(), Box<dyn std::error::Error>> {
        let mut loader = MongoLoader::new("mongodb://localhost:27017".to_owned());
        loader.init().await?;

        let info = loader.load_stock_info(doc! {}, doc! {}, Some(1)).await?;

        println!("info: {:?}", info);

        let nd = NaiveDate::parse_from_str("2022-12-12", "%Y-%m-%d").unwrap();
        let data = loader
            .load_stock_daily(
                doc! {"code": "sz001219", "trade_date": {"$lte": loader.naive_date_to_datetime_str(&nd)?}},
                doc! {"trade_date": -1},
                Some(2),
            )
            .await?;
        println!("data: {:?}", data);
        Ok(())
    }
}
