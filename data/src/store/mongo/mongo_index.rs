use mongodb::{bson::doc, Client, IndexModel};

use crate::store::{
    DATABASE, TAB_BOND_DAILY, TAB_BOND_INFO, TAB_FUND_DAILY, TAB_FUND_INFO, TAB_FUND_NET,
    TAB_INDEX_DAILY, TAB_INDEX_INFO, TAB_STOCK_CONCEPT, TAB_STOCK_CONCEPT_DAILY,
    TAB_STOCK_CONCEPT_DETAIL, TAB_STOCK_DAILY, TAB_STOCK_INDEX, TAB_STOCK_INDUSTRY,
    TAB_STOCK_INDUSTRY_DAILY, TAB_STOCK_INDUSTRY_DETAIL, TAB_STOCK_INFO, TAB_STOCK_MARGIN,
    TAB_STOCK_YJBB, TAB_TRADE_DATE,
};
use crate::{Error, Result};

pub(crate) async fn build_index(client: Client) -> Result<()> {
    let db = client.database(DATABASE);
    log::info!("start build index!");
    let indexes = vec![
        IndexModel::builder().keys(doc! {"trade_date": -1}).build(),
        IndexModel::builder().keys(doc! {"code": 1}).build(),
        IndexModel::builder()
            .keys(doc! {"trade_date": -1, "code": 1})
            .build(),
    ];
    // trade_date
    {
        log::info!("start build {} index!", TAB_TRADE_DATE);
        let coll = db.collection::<rwqfetch::TradeDate>(TAB_TRADE_DATE);
        coll.create_index(
            IndexModel::builder().keys(doc! {"trade_date": -1}).build(),
            None,
        )
        .await
        .map_err(|e| {
            log::error!("create index err: {}", e.to_string());
            Error::Custom(format!("create index err: {}", e.to_string()))
        })?;
    }
    // fund
    {
        log::info!("start build {} index!", TAB_FUND_DAILY);
        let coll = db.collection::<rwqfetch::Bar>(TAB_FUND_DAILY);
        coll.create_indexes(indexes.clone(), None)
            .await
            .map_err(|e| {
                log::error!("create index err: {}", e.to_string());
                Error::Custom(format!("create index err: {}", e.to_string()))
            })?;

        log::info!("start build {} index!", TAB_FUND_NET);

        let coll = db.collection::<rwqfetch::FundNet>(TAB_FUND_NET);
        coll.create_indexes(indexes.clone(), None)
            .await
            .map_err(|e| {
                log::error!("create index err: {}", e.to_string());
                Error::Custom(format!("create index err: {}", e.to_string()))
            })?;

        log::info!("start build {} index!", TAB_FUND_INFO);
        let coll = db.collection::<rwqfetch::FundInfo>(TAB_FUND_INFO);
        coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
            .await
            .map_err(|e| {
                log::error!("create index err: {}", e.to_string());
                Error::Custom(format!("create index err: {}", e.to_string()))
            })?;
    }

    // bond
    {
        log::info!("start build {} index!", TAB_BOND_DAILY);
        let coll = db.collection::<rwqfetch::Bar>(TAB_BOND_DAILY);
        coll.create_indexes(indexes.clone(), None)
            .await
            .map_err(|e| {
                log::error!("create index err: {}", e.to_string());
                Error::Custom(format!("create index err: {}", e.to_string()))
            })?;

        log::info!("start build {} index!", TAB_BOND_INFO);
        let coll = db.collection::<rwqfetch::BondInfo>(TAB_BOND_INFO);
        coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
            .await
            .map_err(|e| {
                log::error!("create index err: {}", e.to_string());
                Error::Custom(format!("create index err: {}", e.to_string()))
            })?;
    }

    // stock
    {
        {
            // index
            log::info!("start build {} index!", TAB_INDEX_DAILY);
            let coll = db.collection::<rwqfetch::Bar>(TAB_INDEX_DAILY);
            coll.create_indexes(indexes.clone(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;

            log::info!("start build {} index!", TAB_INDEX_INFO);
            let coll = db.collection::<rwqfetch::StockInfo>(TAB_INDEX_INFO);
            coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;
        }
        {
            // stock
            log::info!("start build {} index!", TAB_STOCK_DAILY);
            let coll = db.collection::<rwqfetch::Bar>(TAB_STOCK_DAILY);
            coll.create_indexes(indexes.clone(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;

            log::info!("start build {} index!", TAB_STOCK_INFO);
            let coll = db.collection::<rwqfetch::StockInfo>(TAB_STOCK_INFO);
            coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;

            log::info!("start build {} index!", TAB_STOCK_MARGIN);
            let coll = db.collection::<rwqfetch::StockMargin>(TAB_STOCK_MARGIN);
            coll.create_indexes(indexes.clone(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;
            log::info!("start build {} index!", TAB_STOCK_INDEX);
            let coll = db.collection::<rwqfetch::StockIndex>(TAB_STOCK_INDEX);
            coll.create_indexes(indexes.clone(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;

            log::info!("start build {} index!", TAB_STOCK_YJBB);
            let coll = db.collection::<rwqfetch::StockYJBB>(TAB_STOCK_YJBB);
            coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;

            {
                // concept
                log::info!("start build {} index!", TAB_STOCK_CONCEPT_DAILY);
                let coll = db.collection::<rwqfetch::Bar>(TAB_STOCK_CONCEPT_DAILY);
                coll.create_indexes(indexes.clone(), None)
                    .await
                    .map_err(|e| {
                        log::error!("create index err: {}", e.to_string());
                        Error::Custom(format!("create index err: {}", e.to_string()))
                    })?;

                log::info!("start build {} index!", TAB_STOCK_CONCEPT);
                let coll = db.collection::<rwqfetch::StockConcept>(TAB_STOCK_CONCEPT);
                coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
                    .await
                    .map_err(|e| {
                        log::error!("create index err: {}", e.to_string());
                        Error::Custom(format!("create index err: {}", e.to_string()))
                    })?;

                log::info!("start build {} index!", TAB_STOCK_CONCEPT_DETAIL);
                let coll = db.collection::<rwqfetch::StockConcept>(TAB_STOCK_CONCEPT_DETAIL);
                coll.create_index(
                    IndexModel::builder()
                        .keys(doc! {"code": 1, "stock_code": 1})
                        .build(),
                    None,
                )
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;
            }
            {
                // industry
                log::info!("start build {} index!", TAB_STOCK_INDUSTRY_DAILY);
                let coll = db.collection::<rwqfetch::Bar>(TAB_STOCK_INDUSTRY_DAILY);
                coll.create_indexes(indexes.clone(), None)
                    .await
                    .map_err(|e| {
                        log::error!("create index err: {}", e.to_string());
                        Error::Custom(format!("create index err: {}", e.to_string()))
                    })?;

                log::info!("start build {} index!", TAB_STOCK_INDUSTRY);
                let coll = db.collection::<rwqfetch::StockIndustry>(TAB_STOCK_INDUSTRY);
                coll.create_index(IndexModel::builder().keys(doc! {"code": 1}).build(), None)
                    .await
                    .map_err(|e| {
                        log::error!("create index err: {}", e.to_string());
                        Error::Custom(format!("create index err: {}", e.to_string()))
                    })?;

                log::info!("start build {} index!", TAB_STOCK_INDUSTRY_DETAIL);
                let coll = db.collection::<rwqfetch::StockConcept>(TAB_STOCK_INDUSTRY_DETAIL);
                coll.create_index(
                    IndexModel::builder()
                        .keys(doc! {"code": 1, "stock_code": 1})
                        .build(),
                    None,
                )
                .await
                .map_err(|e| {
                    log::error!("create index err: {}", e.to_string());
                    Error::Custom(format!("create index err: {}", e.to_string()))
                })?;
            }
        }
    }

    Ok(())
}
