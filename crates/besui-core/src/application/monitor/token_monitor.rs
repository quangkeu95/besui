// use tracing::info;

// use crate::{
//     application::{fetcher::TokenFetcherService, store::RootStore},
//     database::DbConnection,
//     domain::token::store::TokenStorage,
//     resolver::RootResolver,
//     services::token_fetcher::TokenFetcher,
//     utils::coingecko::CoingeckoClient,
// };
// use std::sync::Arc;

// pub struct TokenMonitor {
//     token_fetcher: TokenFetcher,
//     token_storage: TokenStorage,
//     db_conn: DbConnection,
// }

// impl TokenMonitor {
//     pub fn new() -> Self {
//         let root_resolver = RootResolver::get().unwrap();
//         let db_conn = root_resolver.db_conn.clone();

//         let coingecko_client = Arc::new(CoingeckoClient::default());

//         let token_fetcher =
//             Arc::new(TokenFetcherService::new(coingecko_client.clone())) as TokenFetcher;
//         let token_storage: TokenStorage = Arc::new(RootStore::default());

//         TokenMonitor {
//             token_fetcher,
//             token_storage,
//             db_conn,
//         }
//     }

//     pub async fn start(&self) -> anyhow::Result<()> {
//         info!("Starting token monitor...");
//         Ok(())
//     }
// }
