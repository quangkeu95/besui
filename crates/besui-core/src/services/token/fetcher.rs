use crate::{
    domain::token::{service::TokenFetcher, Token, TokenWithPair},
    utils::coingecko::CoingeckoClient,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct TokenFetcherService {
    coingecko_client: Arc<CoingeckoClient>,
}

impl Default for TokenFetcherService {
    fn default() -> Self {
        TokenFetcherService {
            coingecko_client: Arc::new(CoingeckoClient::default()),
        }
    }
}

#[async_trait]
impl TokenFetcher for TokenFetcherService {
    async fn fetch_all_token_ids(&self) -> anyhow::Result<Vec<Token>> {
        todo!()
    }

    async fn fetch_token_pair_by_exchange(
        &self,
        exchange_id: String,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TokenWithPair>> {
        todo!()
    }
}
