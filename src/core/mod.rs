use std::sync::Arc;

use crate::database::new_database_connection;
use sea_orm::DbConn;

pub struct Core {
    db_conn: Arc<DbConn>,
}

impl Core {
    pub async fn new() -> Core {
        let db_conn = new_database_connection().await.unwrap();

        Core {
            db_conn: Arc::new(db_conn),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
