/* use crate::store::Store;
use async_trait::async_trait;



pub(crate) struct MysqlStore {

}

impl MysqlStore {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl Store for MysqlStore {
    async fn init(&self) -> Result<()> {
        todo!()
    }
    fn syncer_types(&self) -> Result<Vec<SyncDataType>> {
        todo!()
    }
    fn syncer(&self, typ: &SyncDataType) -> Result<Arc<Box<dyn Syncer>>> {
        todo!()
    }
} */
