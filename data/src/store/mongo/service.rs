use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Client,
};
use serde::de::DeserializeOwned;

use crate::{store::DATABASE, Error, Result};

pub(crate) async fn insert_many<T>(
    client: Client,
    collection: &str,
    info: &Vec<T>,
    del_old: bool,
) -> Result<()>
where
    T: serde::ser::Serialize,
{
    let db = client.database(DATABASE);
    let coll = db.collection::<T>(collection);

    if del_old {
        let del_res = coll.delete_many(doc! {}, None).await.map_err(|e| {
            log::error!("delete collection {} failed: {}", collection, e.to_string());
            Error::Custom(format!(
                "delete collection {} failed: {}",
                collection,
                e.to_string()
            ))
        })?;
        log::info!("delete {}, {} items", collection, del_res.deleted_count);
    }

    log::info!("insert into {}, {} items", collection, info.len());
    if info.len() == 1 {
        let item = info.get(0).unwrap();
        coll.insert_one(item, None).await.map_err(|e| {
            log::error!("insert collection {} failed: {}", collection, e.to_string());
            Error::Custom(format!(
                "insert collection {} failed: {}",
                collection,
                e.to_string()
            ))
        })?;
    } else {
        coll.insert_many(info, None).await.map_err(|e| {
            log::error!("insert collection {} failed: {}", collection, e.to_string());
            Error::Custom(format!(
                "insert collection {} failed: {}",
                collection,
                e.to_string()
            ))
        })?;
    }
    Ok(())
}

pub async fn query<T>(
    client: Client,
    collection: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
) -> Result<Vec<T>>
where
    T: DeserializeOwned + Unpin + Send + Sync,
{
    let db = client.database(DATABASE);
    let coll = db.collection::<T>(collection);

    let mut res = coll.find(filter, options).await.map_err(|e| {
        log::error!("find {} error: {}", collection, e.to_string());
        Error::Custom(format!("find {} error: {}", collection, e.to_string()))
    })?;
    let mut data = Vec::new();
    while let Some(info) = res.try_next().await.map_err(|e| {
        log::error!("try next {}  error: {}", collection, e.to_string());
        Error::Custom(format!("try next {}  error: {}", collection, e.to_string()))
    })? {
        data.push(info);
    }

    Ok(data)
}

pub async fn query_one<T>(
    client: Client,
    collection: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
) -> Result<Option<T>>
where
    T: DeserializeOwned + Unpin + Send + Sync + Clone,
{
    let db = client.database(DATABASE);
    let coll = db.collection::<T>(collection);

    let mut res = coll.find(filter, options).await.map_err(|e| {
        log::error!("find {} error: {}", collection, e.to_string());
        Error::Custom(format!("find {} error: {}", collection, e.to_string()))
    })?;
    let data = res.try_next().await.map_err(|e| {
        log::error!("try next {}  error: {}", collection, e.to_string());
        Error::Custom(format!("try next {}  error: {}", collection, e.to_string()))
    })?;
    Ok(data)
}
