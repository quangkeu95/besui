use async_trait::async_trait;
use std::sync::Arc;

use super::Exchange;

#[async_trait]
pub trait ExchangeFetcher {
    async fn get_list_exchanges(&self) -> anyhow::Result<Vec<Exchange>>;
}

pub type SharedExchangeFetcher = Arc<dyn ExchangeFetcher + Send + Sync>;
