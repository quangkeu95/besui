use std::sync::Arc;

use crate::database::DbConnection;
use crate::services::ServiceRegister;

pub struct BesuiCore {
    pub db_conn: DbConnection,
    pub service_register: ServiceRegister,
}

impl BesuiCore {
    pub fn new(db_conn: DbConnection) -> Self {
        let service_register = ServiceRegister::new(db_conn.clone());
        BesuiCore {
            db_conn,
            service_register,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
