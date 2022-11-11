use crate::{
    domain::exchange::Exchange, services::exchange_fetcher::ExchangeFetcherTrait,
    utils::coingecko::CoingeckoClient,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ExchangeFetcherService {
    coingecko_client: Arc<CoingeckoClient>,
}

impl ExchangeFetcherService {
    pub fn new(coingecko_client: Arc<CoingeckoClient>) -> Self {
        ExchangeFetcherService { coingecko_client }
    }
}

#[async_trait]
impl ExchangeFetcherTrait for ExchangeFetcherService {
    async fn get_list_exchanges(&self) -> anyhow::Result<Vec<Exchange>> {
        self.coingecko_client.get_list_exchanges().await
    }
}
