use std::time::Duration;

use super::*;
use anyhow::{bail, Context};
use async_trait::async_trait;
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::*, ConnectOptions, Database};
use std::sync::Arc;
use tracing::log::LevelFilter;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_CONNECTIONS: u32 = 200;

#[derive(Clone)]
pub struct PostgresPersistence {
    db_url: String,
    log_enabled: bool,
    log_level: LevelFilter,
}

impl Default for PostgresPersistence {
    fn default() -> Self {
        PostgresPersistence {
            db_url: "".to_owned(),
            log_enabled: true,
            log_level: LevelFilter::Info,
        }
    }
}

impl PostgresPersistence {
    pub fn new(db_url: String, log_enabled: bool, log_level: LevelFilter) -> Self {
        PostgresPersistence {
            db_url,
            log_enabled,
            log_level,
        }
    }

    pub async fn run_migration(&self, connection: SharedConnection) -> anyhow::Result<()> {
        let postgres_conn = Self::downcast_ref(connection.as_ref())?;
        Migrator::up(postgres_conn, None)
            .await
            .context("Error running postgres migration")
    }

    pub fn downcast_ref<'borrow>(
        connection: &'borrow dyn ConnectionPool,
    ) -> anyhow::Result<&'borrow PostgresConnection> {
        if let Some(postgres_conn) = connection.downcast_ref::<PostgresConnection>() {
            Ok(postgres_conn)
        } else {
            bail!("Error down casting to postgres connection")
        }
    }
}

#[async_trait]
impl Persistence for PostgresPersistence {
    async fn new_connection_pool(&self) -> anyhow::Result<SharedConnection> {
        let mut opt = ConnectOptions::new(self.db_url.clone());
        opt.connect_timeout(CONNECT_TIMEOUT)
            .acquire_timeout(ACQUIRE_TIMEOUT)
            .max_connections(MAX_CONNECTIONS)
            .sqlx_logging(self.log_enabled)
            .sqlx_logging_level(self.log_level);
        let db_conn = Database::connect(opt).await?;
        Ok(Arc::new(db_conn))
    }
}
pub type PostgresConnection = DatabaseConnection;

impl ConnectionPool for PostgresConnection {}
