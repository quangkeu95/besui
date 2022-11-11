use tracing::info;

use crate::{
    application::{fetcher::ExchangeFetcherService, store::Mutation},
    domain::exchange::store::ExchangeStorage,
    resolver::RootResolver,
    services::exchange_fetcher::ExchangeFetcher,
    utils::coingecko::CoingeckoClient,
};
use std::sync::Arc;

pub struct ExchangeMonitor {
    exchange_fetcher: ExchangeFetcher,
    exchange_storage: ExchangeStorage,
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
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!("starting exchange monitor...");

        // let mut handles: Vec<tokio::task::JoinHandle<anyhow::Result<()>>> = vec![];

        // handles.push(tokio::spawn(async move {
        //     let exchange_fetcher = self.exchange_fetcher.clone();
        //     let exchanges = exchange_fetcher.get_list_exchanges().await?;

        //     Ok(())
        // }));

        // // futures::future::join_all(handles).await?;

        Ok(())
    }
}
