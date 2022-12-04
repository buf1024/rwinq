use hiq_data::store::mongo::{query, query_one};
use hiq_data::store::{
    TAB_BOND_DAILY, TAB_BOND_INFO, TAB_FUND_DAILY, TAB_FUND_INFO, TAB_FUND_NET, TAB_INDEX_DAILY,
    TAB_INDEX_INFO, TAB_STOCK_CONCEPT, TAB_STOCK_CONCEPT_DAILY, TAB_STOCK_CONCEPT_DETAIL,
    TAB_STOCK_DAILY, TAB_STOCK_INDEX, TAB_STOCK_INDUSTRY, TAB_STOCK_INDUSTRY_DAILY,
    TAB_STOCK_INDUSTRY_DETAIL, TAB_STOCK_INFO, TAB_STOCK_MARGIN, TAB_STOCK_YJBB,
};
use hiq_pycommon::runtime;
use mongodb::bson::{doc, Document};
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::Client;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use serde::de::DeserializeOwned;

fn build_options(
    filter: Option<String>,
    sort: Option<String>,
    limit: Option<i64>,
) -> PyResult<(Document, FindOptions)> {
    let filter: Document = if let Some(filter) = filter {
        serde_json::from_str(&filter).map_err(|e| PyException::new_err(e.to_string()))?
    } else {
        doc! {}
    };
    let sort: Document = if let Some(sort) = sort {
        serde_json::from_str(&sort).map_err(|e| PyException::new_err(e.to_string()))?
    } else {
        doc! {}
    };

    let options = if let Some(limit) = limit {
        FindOptions::builder().sort(sort).limit(limit).build()
    } else {
        FindOptions::builder().sort(sort).build()
    };

    Ok((filter, options))
}

async fn connect(url: String) -> PyResult<Client> {
    let mut client_options = ClientOptions::parse(&url[..])
        .await
        .map_err(|e| PyException::new_err(e.to_string()))?;

    client_options.app_name = Some("HIQ App".to_string());

    let client =
        Client::with_options(client_options).map_err(|e| PyException::new_err(e.to_string()))?;
    Ok(client)
}

async fn load_data<T, R>(
    client: Client,
    collection: &str,
    filter: Option<String>,
    sort: Option<String>,
    limit: Option<i64>,
) -> PyResult<Vec<R>>
where
    T: DeserializeOwned + Unpin + Send + Sync + Clone,
    R: From<T>,
{
    let (filter, options) = build_options(filter, sort, limit)?;
    let data: Vec<T> = if limit.is_some() {
        query(client, collection, filter, options)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?
    } else {
        let data: Option<T> = query_one(client, collection, filter, options)
            .await
            .map_err(|e| PyException::new_err(e.to_string()))?;
        data.map_or(Vec::new(), |e| vec![e])
    };
    let data: Vec<_> = data.into_iter().map(R::from).collect();
    Ok(data)
}

#[pyclass]
pub struct MongoFetch {
    client: Option<Client>,
}

#[pymethods]
impl MongoFetch {
    #[new]
    fn new<'a>(py: Python<'a>, url: String) -> PyResult<Self> {
        let event_loop = py.import("asyncio")?.call_method0("new_event_loop")?;
        let client = pyo3_asyncio::tokio::run_until_complete(event_loop, connect(url))?;
        Ok(Self {
            client: Some(client),
        })
    }

    fn load_bond_info<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::BondInfo, hiq_pycommon::BondInfo>(
                client,
                TAB_BOND_INFO,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
    fn load_bond_daily<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
                client,
                TAB_BOND_DAILY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_fund_info<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::FundInfo, hiq_pycommon::FundInfo>(
                client,
                TAB_FUND_INFO,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
    fn load_fund_daily<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
                client,
                TAB_FUND_DAILY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
    fn load_fund_net<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::FundNet, hiq_pycommon::FundNet>(
                client,
                TAB_FUND_NET,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_index_info<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockInfo, hiq_pycommon::StockInfo>(
                client,
                TAB_INDEX_INFO,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_index_daily<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
                client,
                TAB_INDEX_DAILY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_info<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockInfo, hiq_pycommon::StockInfo>(
                client,
                TAB_STOCK_INFO,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_daily<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
                client,
                TAB_STOCK_DAILY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_index<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockIndex, hiq_pycommon::StockIndex>(
                client,
                TAB_STOCK_INDEX,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
    fn load_stock_industry<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockIndustry, hiq_pycommon::StockIndustry>(
                client,
                TAB_STOCK_INDUSTRY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_industry_daily<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
                client,
                TAB_STOCK_INDUSTRY_DAILY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
    fn load_stock_industry_detail<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockIndustryDetail, hiq_pycommon::StockIndustryDetail>(
                client,
                TAB_STOCK_INDUSTRY_DETAIL,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_concept<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockConcept, hiq_pycommon::StockConcept>(
                client,
                TAB_STOCK_CONCEPT,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_concept_daily<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
                client,
                TAB_STOCK_CONCEPT_DAILY,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
    fn load_stock_concept_detail<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockConceptDetail, hiq_pycommon::StockConceptDetail>(
                client,
                TAB_STOCK_CONCEPT_DETAIL,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_yjbb<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockYJBB, hiq_pycommon::StockYJBB>(
                client,
                TAB_STOCK_YJBB,
                filter,
                sort,
                limit,
            )
            .await
        })
    }

    fn load_stock_margin<'a>(
        &self,
        py: Python<'a>,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<&'a PyAny> {
        let client = self.client.clone().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            load_data::<hiq_common::StockMargin, hiq_pycommon::StockMargin>(
                client,
                TAB_STOCK_MARGIN,
                filter,
                sort,
                limit,
            )
            .await
        })
    }
}

#[pyclass]
pub struct BlockMongoFetch {
    client: Option<Client>,
}

#[pymethods]
impl BlockMongoFetch {
    #[new]
    fn new<'a>(py: Python<'a>, url: String) -> PyResult<Self> {
        let event_loop = py.import("asyncio")?.call_method0("new_event_loop")?;
        let client = pyo3_asyncio::tokio::run_until_complete(event_loop, connect(url))?;
        Ok(Self {
            client: Some(client),
        })
    }

    fn load_bond_info(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::BondInfo>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::BondInfo, hiq_pycommon::BondInfo>(
            client,
            TAB_BOND_INFO,
            filter,
            sort,
            limit,
        ))
    }
    fn load_bond_daily(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::Bar>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
            client,
            TAB_BOND_DAILY,
            filter,
            sort,
            limit,
        ))
    }

    fn load_fund_info(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::FundInfo>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::FundInfo, hiq_pycommon::FundInfo>(
            client,
            TAB_FUND_INFO,
            filter,
            sort,
            limit,
        ))
    }
    fn load_fund_daily(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::Bar>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
            client,
            TAB_FUND_DAILY,
            filter,
            sort,
            limit,
        ))
    }
    fn load_fund_net(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::FundNet>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::FundNet, hiq_pycommon::FundNet>(
            client,
            TAB_FUND_NET,
            filter,
            sort,
            limit,
        ))
    }

    fn load_index_info(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockInfo>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::StockInfo, hiq_pycommon::StockInfo>(
            client,
            TAB_INDEX_INFO,
            filter,
            sort,
            limit,
        ))
    }

    fn load_index_daily(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::Bar>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
            client,
            TAB_INDEX_DAILY,
            filter,
            sort,
            limit,
        ))
    }

    fn load_stock_info(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockInfo>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::StockInfo, hiq_pycommon::StockInfo>(
            client,
            TAB_STOCK_INFO,
            filter,
            sort,
            limit,
        ))
    }

    fn load_stock_daily(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::Bar>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
            client,
            TAB_STOCK_DAILY,
            filter,
            sort,
            limit,
        ))
    }

    fn load_stock_index(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockIndex>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(
            load_data::<hiq_common::StockIndex, hiq_pycommon::StockIndex>(
                client,
                TAB_STOCK_INDEX,
                filter,
                sort,
                limit,
            ),
        )
    }
    fn load_stock_industry(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockIndustry>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<
            hiq_common::StockIndustry,
            hiq_pycommon::StockIndustry,
        >(client, TAB_STOCK_INDUSTRY, filter, sort, limit))
    }

    fn load_stock_industry_daily(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::Bar>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
            client,
            TAB_STOCK_INDUSTRY_DAILY,
            filter,
            sort,
            limit,
        ))
    }
    fn load_stock_industry_detail(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockIndustryDetail>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<
            hiq_common::StockIndustryDetail,
            hiq_pycommon::StockIndustryDetail,
        >(
            client, TAB_STOCK_INDUSTRY_DETAIL, filter, sort, limit
        ))
    }

    fn load_stock_concept(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockConcept>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<
            hiq_common::StockConcept,
            hiq_pycommon::StockConcept,
        >(client, TAB_STOCK_CONCEPT, filter, sort, limit))
    }

    fn load_stock_concept_daily(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::Bar>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::Bar, hiq_pycommon::Bar>(
            client,
            TAB_STOCK_CONCEPT_DAILY,
            filter,
            sort,
            limit,
        ))
    }
    fn load_stock_concept_detail(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockConceptDetail>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<
            hiq_common::StockConceptDetail,
            hiq_pycommon::StockConceptDetail,
        >(
            client, TAB_STOCK_CONCEPT_DETAIL, filter, sort, limit
        ))
    }

    fn load_stock_yjbb(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockYJBB>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<hiq_common::StockYJBB, hiq_pycommon::StockYJBB>(
            client,
            TAB_STOCK_YJBB,
            filter,
            sort,
            limit,
        ))
    }

    fn load_stock_margin(
        &self,
        filter: Option<String>,
        sort: Option<String>,
        limit: Option<i64>,
    ) -> PyResult<Vec<hiq_pycommon::StockMargin>> {
        let client = self.client.clone().unwrap();
        runtime()?.block_on(load_data::<
            hiq_common::StockMargin,
            hiq_pycommon::StockMargin,
        >(client, TAB_STOCK_MARGIN, filter, sort, limit))
    }
}
