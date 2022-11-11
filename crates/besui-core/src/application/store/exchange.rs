use crate::{
    database::DbConnection,
    domain::exchange::{store::ExchangeStore, Exchange},
};
use ::entity::exchange;
use async_trait::async_trait;
use sea_orm::*;

use super::Mutation;

// #[derive(Clone)]
// pub struct PostgresExchangeStorage {
//     db_conn: DbConnection,
// }

// impl PostgresExchangeStorage {
//     pub fn new(db_conn: DbConnection) -> Self {
//         PostgresExchangeStorage { db_conn }
//     }
// }

// #[async_trait]
// impl ExchangeStore for PostgresExchangeStorage {
//     async fn save_list_exchanges(&self, list_exchange: Vec<Exchange>) -> anyhow::Result<()> {
//         let insert_items = list_exchange
//             .iter()
//             .map(|item| {
//                 let exchange_item = exchange::ActiveModel {
//                     id: Set(item.id.clone()),
//                     name: Set(item.name.clone()),
//                 };
//                 exchange_item
//             })
//             .collect::<Vec<exchange::ActiveModel>>();

//         let db = self.db_conn.get_raw_connection();
//         let _ = exchange::Entity::insert_many(insert_items)
//             .exec(db.as_ref())
//             .await?;
//         Ok(())
//     }
// }

#[async_trait]
impl ExchangeStore for Mutation {
    async fn save_list_exchanges(
        &self,
        db_conn: DbConnection,
        list_exchanges: Vec<Exchange>,
    ) -> anyhow::Result<()> {
        let insert_items = list_exchanges
            .iter()
            .map(|item| {
                let exchange_item = exchange::ActiveModel {
                    id: Set(item.id.clone()),
                    name: Set(item.name.clone()),
                };
                exchange_item
            })
            .collect::<Vec<exchange::ActiveModel>>();

        let db = db_conn.get_raw_connection();
        let _ = exchange::Entity::insert_many(insert_items)
            .exec(db.as_ref())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod exchange_store_test {
    use ::entity::exchange;
    use claims::assert_ok;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    use crate::{
        application::store::Mutation,
        database::DbConnection,
        domain::exchange::{store::ExchangeStore, Exchange},
    };

    fn mock_db_connection() -> DbConnection {
        let raw_db_conn = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![])
            .into_connection();
        let db_conn_res = DbConnection::try_from(raw_db_conn);
        let db_conn = assert_ok!(db_conn_res);
        db_conn
    }

    fn mock_db_conn_with_list_exchange(list_exchange: Vec<Exchange>) -> DbConnection {
        let query_results = list_exchange
            .into_iter()
            .map(|item| {
                vec![exchange::Model {
                    id: item.id.clone(),
                    name: item.name.clone(),
                }]
            })
            .collect::<Vec<Vec<exchange::Model>>>();
        let raw_db_conn = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(query_results)
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let db_conn_res = DbConnection::try_from(raw_db_conn);
        let db_conn = assert_ok!(db_conn_res);
        db_conn
    }

    #[tokio::test]
    async fn test_save_list_exchange() {
        let list_exchanges = vec![Exchange {
            id: "binance".to_owned(),
            name: "Binance".to_owned(),
        }];

        let db_conn = mock_db_conn_with_list_exchange(list_exchanges.clone());
        let mutation = Mutation::default();

        let res = mutation
            .save_list_exchanges(db_conn.clone(), list_exchanges)
            .await;
        assert_ok!(res);
    }
}
