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
    fn syncer_types(&self) -> Result<Vec<HiqSyncDataType>> {
        todo!()
    }
    fn syncer(&self, typ: &HiqSyncDataType) -> Result<Arc<Box<dyn Syncer>>> {
        todo!()
    }
} */