use crate::{
    domain::exchange::{services::ExchangeFetcher, Exchange},
    utils::coingecko::CoingeckoClient,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ExchangeFetcherService {
    coingecko_client: Arc<CoingeckoClient>,
}

#[async_trait]
impl ExchangeFetcher for ExchangeFetcherService {
    async fn get_list_exchanges(&self) -> anyhow::Result<Vec<Exchange>> {
        self.coingecko_client.get_list_exchanges().await
    }
}
