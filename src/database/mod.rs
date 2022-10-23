use crate::config::get_global_config;
use sea_orm::{ConnectOptions, Database, DbConn};
use std::env;
use tracing::log;

mod query;
pub use query::*;

pub async fn new_database_connection() -> anyhow::Result<DbConn> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let conn = Database::connect(opt).await?;
    Ok(conn)
}
