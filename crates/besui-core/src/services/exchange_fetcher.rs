use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::exchange::Exchange;

#[async_trait]
pub trait ExchangeFetcherTrait {
    async fn get_list_exchanges(&self) -> anyhow::Result<Vec<Exchange>>;
}

pub type ExchangeFetcher = Arc<dyn ExchangeFetcherTrait + Send + Sync>;
