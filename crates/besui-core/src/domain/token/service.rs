// use crate::services::Service;

// use super::store::TokenStorage;
// use async_trait::async_trait;
// use tracing::info;

// pub struct TokenService {
//     storage: TokenStorage,
// }

// impl TokenService {
//     pub fn new(storage: TokenStorage) -> Self {
//         TokenService { storage }
//     }
// }

// #[async_trait]
// impl Service for TokenService {
//     async fn on_service_started(&self) -> anyhow::Result<()> {
//         info!("Starting TokenService...");
//         Ok(())
//     }
// }

use super::*;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TokenFetcher {
    async fn fetch_all_token_ids(&self) -> anyhow::Result<Vec<Token>>;
    /**
     * Fetch all tokens on specific exchanges
     * Omit the page parameter if you want to fetch all tokens
     * Number of tokens per page is 100 for default
     */
    async fn fetch_token_pair_by_exchange(
        &self,
        exchange_id: String,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TokenWithPair>>;
}

pub type SharedTokenFetcher = Arc<dyn TokenFetcher + Send + Sync>;
