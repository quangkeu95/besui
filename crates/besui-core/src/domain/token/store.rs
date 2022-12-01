use std::sync::Arc;

use async_trait::async_trait;

use crate::persistence::SharedConnection;

use super::{filter::TokenFilter, Token, TokenId};

#[async_trait]
pub trait TokenStore {
    async fn save_token(&self, db_conn: SharedConnection, token: Token) -> anyhow::Result<()>;
    async fn save_list_tokens(
        &self,
        db_conn: SharedConnection,
        tokens: Vec<Token>,
    ) -> anyhow::Result<()>;
    async fn get_token_by_id(
        &self,
        db_conn: SharedConnection,
        id: &TokenId,
    ) -> anyhow::Result<Token>;
    async fn get_list_tokens(
        &self,
        db_conn: SharedConnection,
        filter: TokenFilter,
    ) -> anyhow::Result<Vec<Token>>;
}

pub type SharedTokenStore = Arc<dyn TokenStore + Send + Sync>;
