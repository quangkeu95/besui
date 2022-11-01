use anyhow::Context;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

use crate::{
    application::store::token::PostgresTokenStorage,
    database::DbConnection,
    domain::token::{service::TokenService, store::TokenStorage},
};

pub struct ServiceRegister {
    pub token_service: TokenService,
}

/// A simple service container responsible for managing the various services our API endpoints will pull from through axum extensions.
impl ServiceRegister {
    pub fn new(db_conn: DbConnection) -> Self {
        info!("Initializing service register...");

        let token_storage = Arc::new(PostgresTokenStorage::new(db_conn.clone())) as TokenStorage;
        let token_service = TokenService::new(token_storage);

        ServiceRegister { token_service }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        self.token_service
            .on_service_started()
            .await
            .context("cannot start token service")?;

        Ok(())
    }
}

#[async_trait]
pub trait Service {
    async fn on_service_started(&self) -> anyhow::Result<()>;
}
