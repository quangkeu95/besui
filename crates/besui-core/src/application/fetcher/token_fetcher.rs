use async_trait::async_trait;

use crate::{domain::token::Token, services::token_fetcher::TokenFetcher};

pub struct TokenFetcherService {}

impl TokenFetcherService {
    pub async fn fetch_all_token_ids() {}
}

#[async_trait]
impl TokenFetcher for TokenFetcherService {
    /** Fetch all tokens info from Coingecko, Coinmarketcap, 3rd party APIs */
    async fn fetch_all_tokens() -> anyhow::Result<Vec<Token>> {
        
        unimplemented!()
    }
}
