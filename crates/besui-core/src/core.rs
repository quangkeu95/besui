use crate::database::{DbConnection, DbConnectionManager};
use crate::resolver::RootResolver;
use crate::services::ServiceRegister;
use anyhow::Context;
use besui_config::config::AppConfig;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;
use tracing::log::LevelFilter;

pub struct BesuiCore {
    pub db_conn: DbConnection,
    pub service_register: ServiceRegister,
}

impl BesuiCore {
    pub async fn new(config: Arc<AppConfig>) -> anyhow::Result<Self> {
        let db_log_enabled = config.database.logging.enabled;
        let db_log_level = LevelFilter::from_str(config.database.logging.level.clone().as_str())?;

        info!(
            "Database log settings: logging_enabled = {}, log_level = {}",
            db_log_enabled, db_log_level
        );

        let db_conn = DbConnectionManager::new_connection(
            config.database.url.clone(),
            db_log_enabled,
            db_log_level,
        )
        .await
        .context("could not initialize db connection")?;

        DbConnectionManager::run_migration(&db_conn)
            .await
            .context("error running migration")?;
        info!("Running migration completed!");

        RootResolver::init(|| RootResolver {
            db_conn: db_conn.clone(),
            config: config.clone(),
        })?;

        let service_register = ServiceRegister::new();
        // let monitor_resolver = MonitorResolver::new(db)
        Ok(BesuiCore {
            db_conn,
            service_register,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
