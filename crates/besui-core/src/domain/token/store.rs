use std::sync::Arc;

use async_trait::async_trait;

use super::{filter::TokenFilter, Token, TokenId};

#[async_trait]
pub trait TokenStore {
    async fn save_token(&self, token: Token) -> anyhow::Result<()>;
    async fn save_list_tokens(&self, tokens: Vec<Token>) -> anyhow::Result<()>;
    async fn get_token_by_id(&self, id: &TokenId) -> anyhow::Result<Token>;
    async fn get_list_tokens(&self, filter: TokenFilter) -> anyhow::Result<Vec<Token>>;
}

pub type TokenStorage = Arc<dyn TokenStore + Send + Sync>;
