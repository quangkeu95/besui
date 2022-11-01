use crate::{
    database::DbConnection,
    domain::token::{
        filter::TokenFilter,
        store::{TokenStorage, TokenStore},
        Token, TokenId,
    },
    errors::Error,
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct PostgresTokenStorage {
    db_conn: DbConnection,
}

impl PostgresTokenStorage {
    pub fn new(db_conn: DbConnection) -> Self {
        PostgresTokenStorage { db_conn }
    }
}

#[async_trait]
impl TokenStore for PostgresTokenStorage {
    async fn save_token(&self, token: Token) -> anyhow::Result<()> {
        Ok(())
    }

    async fn save_list_tokens(&self, tokens: Vec<Token>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_token_by_id(&self, id: &TokenId) -> anyhow::Result<Token> {
        unimplemented!()
    }

    async fn get_list_tokens(&self, filter: TokenFilter) -> anyhow::Result<Vec<Token>> {
        unimplemented!()
    }
}
