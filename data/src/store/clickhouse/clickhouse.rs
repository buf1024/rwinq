// use std::{sync::{Arc, Mutex}, collections::HashMap};

// use crate::{store::{Store, HiqCache}, types::{HiqSyncDataType, HiqSyncData}, syncer::Syncer};
// use async_trait::async_trait;
// use hiq_fetch::BondFetch;
// use crate::{Result, Error};

// pub(crate) struct ClickHouseStore {
//     fetch: Arc<dyn BondFetch>,
//     syncer_map: HashMap<HiqSyncDataType, Arc<Box<dyn Syncer>>>,
//     cache: Arc<Mutex<HiqCache>>,
// }

// impl ClickHouseStore {
//     pub fn new(url: String) -> Self {
//         let fetch = Arc::new(hiq_fetch::bond_fetch());
//         let f: Arc<Box<dyn Syncer>> = Arc::new(Box::new(BondInfo::new(fetch.clone())));

//         let mut syncer_map = HashMap::new();
//         syncer_map.insert(HiqSyncDataType::BondInfo, f);

//         let cache = Arc::new(Mutex::new(HiqCache::new()));
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
//     fn syncer_types(&self) -> Result<Vec<HiqSyncDataType>> {
//         let types = self.syncer_map.keys().map(|k| (*k).clone()).collect();
//         Ok(types)
//     }
//     fn syncer(&self, typ: &HiqSyncDataType) -> Result<Arc<Box<dyn Syncer>>> {
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
//     async fn fetch_raw_data(&self) -> Result<HiqSyncData> {
//         let data = self.fetch.fetch_bond_info().await?;
//         Ok(HiqSyncData::BondInfo(data))
//     }
//     async fn save(&self, data: HiqSyncData) -> Result<()> {
//         println!("saver clickhouse");
//         Ok(())
//     }
// }