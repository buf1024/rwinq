// use std::{sync::{Arc, Mutex}, collections::HashMap};

// use crate::{store::{Store, Cache}, types::{SyncDataType, SyncData}, syncer::Syncer};
// use async_trait::async_trait;
// use rwqfetch::BondFetch;
// use crate::{Result, Error};

// pub(crate) struct ClickHouseStore {
//     fetch: Arc<dyn BondFetch>,
//     syncer_map: HashMap<SyncDataType, Arc<Box<dyn Syncer>>>,
//     cache: Arc<Mutex<Cache>>,
// }

// impl ClickHouseStore {
//     pub fn new(url: String) -> Self {
//         let fetch = Arc::new(rwqfetch::bond_fetch());
//         let f: Arc<Box<dyn Syncer>> = Arc::new(Box::new(BondInfo::new(fetch.clone())));

//         let mut syncer_map = HashMap::new();
//         syncer_map.insert(SyncDataType::BondInfo, f);

//         let cache = Arc::new(Mutex::new(Cache::new()));
//         Self {
//             fetch,
//             syncer_map,
//             cache,
//         }
//     }
// }

// #[async_trait]
// impl Store for ClickHouseStore {
//     async fn init(&self) -> Result<()> {
//         Ok(())
//     }
//     fn syncer_types(&self) -> Result<Vec<SyncDataType>> {
//         let types = self.syncer_map.keys().map(|k| (*k).clone()).collect();
//         Ok(types)
//     }
//     fn syncer(&self, typ: &SyncDataType) -> Result<Arc<Box<dyn Syncer>>> {
//         let v = self.syncer_map
//         .get(&typ)
//         .ok_or(Error::Custom("syncer not found"))?;
//         Ok((*v).clone())
//     }
// }

// struct BondInfo {
//     fetch: Arc<dyn BondFetch>,
// }

// impl BondInfo {
//     pub fn new(fetch: Arc<dyn BondFetch>) -> Self {
//         Self { fetch }
//     }
// }

// #[async_trait]
// impl Syncer for BondInfo {
//     async fn fetch_raw_data(&self) -> Result<SyncData> {
//         let data = self.fetch.fetch_bond_info().await?;
//         Ok(SyncData::BondInfo(data))
//     }
//     async fn save(&self, data: SyncData) -> Result<()> {
//         println!("saver clickhouse");
//         Ok(())
//     }
// }
