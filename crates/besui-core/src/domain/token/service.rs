use crate::services::Service;

use super::store::TokenStorage;
use async_trait::async_trait;
use tracing::info;

pub struct TokenService {
    storage: TokenStorage,
}

impl TokenService {
    pub fn new(storage: TokenStorage) -> Self {
        TokenService { storage }
    }
}

#[async_trait]
impl Service for TokenService {
    async fn on_service_started(&self) -> anyhow::Result<()> {
        info!("Starting TokenService...");
        Ok(())
    }
}
