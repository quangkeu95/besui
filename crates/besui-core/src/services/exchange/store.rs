use crate::{
    domain::exchange::{store::ExchangeStore, Exchange},
    persistence::{
        postgres::{PostgresConnection, PostgresPersistence},
        SharedConnection,
    },
};
use ::entity::exchange;
use async_trait::async_trait;
use sea_orm::*;
use tracing::{error, info};

#[async_trait]
impl ExchangeStore for PostgresPersistence {
    async fn save_list_exchanges(
        &self,
        connection_pool: SharedConnection,
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

        let conn = PostgresPersistence::downcast_ref(connection_pool.as_ref())?;
        let _ = exchange::Entity::insert_many(insert_items)
            .on_conflict(
                sea_query::OnConflict::column(exchange::Column::Id)
                    .update_column(exchange::Column::Name)
                    .to_owned(),
            )
            .exec(conn)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod exchange_store_test {
    use ::entity::exchange;
    use claims::assert_ok;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use std::sync::Arc;

    use crate::{
        domain::exchange::{store::ExchangeStore, Exchange},
        persistence::{postgres::PostgresPersistence, SharedConnection},
    };

    fn mock_db_connection() -> SharedConnection {
        let raw_db_conn = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![])
            .into_connection();
        Arc::new(raw_db_conn)
    }

    fn mock_db_conn_with_list_exchange(list_exchange: Vec<Exchange>) -> SharedConnection {
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
        Arc::new(raw_db_conn)
    }

    #[tokio::test]
    async fn test_save_list_exchange() {
        let list_exchanges = vec![Exchange {
            id: "binance".to_owned(),
            name: "Binance".to_owned(),
        }];

        let db_conn = mock_db_conn_with_list_exchange(list_exchanges.clone());
        let persistence = PostgresPersistence::default();

        let res = persistence
            .save_list_exchanges(db_conn.clone(), list_exchanges)
            .await;
        assert_ok!(res);
    }
}
