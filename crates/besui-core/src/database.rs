use sea_orm::{ConnectOptions, Database, DbConn};
use std::{sync::Arc, time::Duration};
use tracing::log;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub struct DbConnection(Arc<DbConn>);

impl DbConnection {
    pub fn new(db: DbConn) -> Self {
        Self(Arc::new(db))
    }
}

pub struct DbConnectionManager {}

/// Handle connection to a database using sea_orm
impl DbConnectionManager {
    pub async fn new_connection(db_url: String) -> anyhow::Result<DbConnection> {
        // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info).connect_timeout(CONNECT_TIMEOUT);

        let conn = Database::connect(opt).await?;
        Ok(DbConnection::new(conn))
    }
}
