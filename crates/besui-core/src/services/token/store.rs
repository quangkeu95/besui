use crate::domain::token::filter::TokenFilter;
use crate::domain::token::{Token, TokenId};
use crate::persistence::SharedConnection;
use crate::{domain::token::store::TokenStore, persistence::postgres::PostgresPersistence};
use async_trait::async_trait;

#[async_trait]
impl TokenStore for PostgresPersistence {
    async fn save_token(&self, db_conn: SharedConnection, token: Token) -> anyhow::Result<()> {
        todo!()
    }

    async fn save_list_tokens(
        &self,
        db_conn: SharedConnection,
        tokens: Vec<Token>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_token_by_id(
        &self,
        db_conn: SharedConnection,
        id: &TokenId,
    ) -> anyhow::Result<Token> {
        todo!()
    }

    async fn get_list_tokens(
        &self,
        db_conn: SharedConnection,
        filter: TokenFilter,
    ) -> anyhow::Result<Vec<Token>> {
        todo!()
    }
}
