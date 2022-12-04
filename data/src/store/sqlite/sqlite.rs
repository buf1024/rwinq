// use std::path::PathBuf;

// use crate::store::Store;
// use async_trait::async_trait;


// pub(crate) struct SqliteStore {

// }

// impl SqliteStore {
//     pub fn new(path: PathBuf) -> Self {
//         Self {  }
//     }
// }

// #[async_trait]
// impl Store for SqliteStore {
//     async fn syncer(&self) -> std::sync::Arc<Vec<Box<dyn crate::syncer::Syncer>>> {
//         todo!()
//     }
// }