use tracing::{debug, error, info};

use crate::{
    application::{fetcher::ExchangeFetcherService, store::Mutation},
    database::DbConnection,
    domain::exchange::store::ExchangeStorage,
    resolver::RootResolver,
    services::exchange_fetcher::ExchangeFetcher,
    utils::coingecko::CoingeckoClient,
};
use std::sync::Arc;
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

pub struct ExchangeMonitor {
    exchange_fetcher: ExchangeFetcher,
    exchange_storage: ExchangeStorage,
    db_conn: DbConnection,
}

impl ExchangeMonitor {
    pub fn new() -> Self {
        let root_resolver = RootResolver::get().unwrap();
        let db_conn = root_resolver.db_conn.clone();

        let coingecko_client = Arc::new(CoingeckoClient::default());
        let exchange_fetcher =
            Arc::new(ExchangeFetcherService::new(coingecko_client.clone())) as ExchangeFetcher;
        let exchange_storage = Arc::new(Mutation::default()) as ExchangeStorage;

        ExchangeMonitor {
            exchange_fetcher,
            exchange_storage,
            db_conn,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!("Starting exchange monitor...");
        const FETCH_LIST_EXCHANGE_RETRY_DURATION: u64 = 1000;
        const FETCH_LIST_EXCHANGE_RETRY_LIMIT: usize = 3;

        let fetch_list_exchanges_retry_strategy =
            ExponentialBackoff::from_millis(FETCH_LIST_EXCHANGE_RETRY_DURATION)
                .map(jitter) // add jitter to delays
                .take(FETCH_LIST_EXCHANGE_RETRY_LIMIT);

        let fetch_list_exchanges_result =
            Retry::spawn(fetch_list_exchanges_retry_strategy, move || async {
                let _ = match self.fetch_and_save_list_exchanges().await {
                    Ok(_) => Ok::<(), anyhow::Error>(()),
                    Err(e) => {
                        error!(
                            "error fetch and save list exchanges: {:?}. Retrying again...",
                            e
                        );
                        return Err(e);
                    }
                };
                Ok(())
            });
        fetch_list_exchanges_result.await?;

        Ok(())
    }

    async fn fetch_and_save_list_exchanges(&self) -> anyhow::Result<()> {
        let fetcher = self.exchange_fetcher.clone();
        let storage = self.exchange_storage.clone();
        let db_conn = self.db_conn.clone();

        let list_exchanges = fetcher.get_list_exchanges().await?;
        debug!("List exchanges fetched: {}", list_exchanges.len());
        let _ = storage.save_list_exchanges(db_conn, list_exchanges).await?;
        debug!("List exchanges saved");
        Ok(())
    }
}
