use crate::persistence::postgres::PostgresPersistence;
use crate::persistence::Persistence;
use crate::resolver::RootResolver;
use anyhow::Context;
use besui_config::AppConfig;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;
use tracing::log::LevelFilter;

pub struct BesuiCore {}

impl BesuiCore {
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        let db_log_enabled = config.database.logging.enabled;
        let db_log_level = LevelFilter::from_str(config.database.logging.level.clone().as_str())?;

        info!(
            "Database log settings: logging_enabled = {}, log_level = {}",
            db_log_enabled, db_log_level
        );

        let postgresPersistence =
            PostgresPersistence::new(config.database.url.clone(), db_log_enabled, db_log_level);
        let connection_pool = postgresPersistence
            .new_connection_pool()
            .await
            .context("Error create persistence connection pool")?;

        postgresPersistence
            .run_migration(connection_pool.clone())
            .await
            .context("Error running migration")?;

        info!("Running migration completed!");

        RootResolver::init(|| RootResolver {
            persistence: Arc::new(postgresPersistence),
            connection_pool: connection_pool.clone(),
        })?;

        Ok(BesuiCore {})
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        // let monitor_resolver = self.monitor_resolver.clone();
        // tokio::spawn(async move { monitor_resolver.start().await });
        Ok(())
    }
}
