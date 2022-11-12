use crate::{
    errors::{DatabaseErrors, Error},
    resolver::RootResolver,
};
use anyhow::{bail, Context};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseBackend, DatabaseConnection};
use std::{str::FromStr, sync::Arc, time::Duration};
use tracing::log::{self, LevelFilter};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub struct DbConnection(Arc<DatabaseConnection>);

impl TryFrom<DatabaseConnection> for DbConnection {
    type Error = Error;

    fn try_from(conn: DatabaseConnection) -> Result<Self, Self::Error> {
        Ok(DbConnection(Arc::new(conn)))
    }
}

impl DbConnection {
    // pub fn mock() -> Self {
    //     let mock_db_conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    //     DbConnection(Arc::new(mock_db_conn))
    // }

    pub fn get_raw_connection(&self) -> Arc<DatabaseConnection> {
        let raw_conn = self.clone().0;
        raw_conn
    }
}

pub struct DbConnectionManager {}

/// Handle connection to a database using sea_orm
impl DbConnectionManager {
    pub async fn new_connection(
        db_url: String,
        log_enabled: bool,
        log_level: LevelFilter,
    ) -> anyhow::Result<DbConnection> {
        // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let mut opt = ConnectOptions::new(db_url);
        opt.connect_timeout(CONNECT_TIMEOUT)
            .sqlx_logging(log_enabled)
            .sqlx_logging_level(log_level);

        let conn = Database::connect(opt).await?;
        let db_conn = DbConnection::try_from(conn)?;
        Ok(db_conn.clone())
    }

    pub async fn run_migration(db_conn: &DbConnection) -> anyhow::Result<()> {
        let conn = db_conn.get_raw_connection();
        Migrator::up(&conn, None)
            .await
            .context("error running database migration")
    }
}
