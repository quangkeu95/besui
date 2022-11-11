use async_trait::async_trait;

use crate::domain::token::Token;

#[async_trait]
pub trait TokenFetcher {
    async fn fetch_all_tokens() -> anyhow::Result<Vec<Token>>;
}
