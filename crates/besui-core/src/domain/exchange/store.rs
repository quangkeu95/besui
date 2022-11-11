use crate::database::DbConnection;

use super::Exchange;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ExchangeStore {
    async fn save_list_exchanges(
        &self,
        db_conn: DbConnection,
        list_exchanges: Vec<Exchange>,
    ) -> anyhow::Result<()>;
}

pub type ExchangeStorage = Arc<dyn ExchangeStore + Send + Sync>;
