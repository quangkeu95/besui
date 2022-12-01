use crate::{
    domain::token::{
        service::{SharedTokenFetcher, TokenFetcher},
        store::SharedTokenStore,
    },
    resolver::RootResolver,
};
use std::sync::Arc;

use super::*;

pub struct TokenService {
    fetcher: SharedTokenFetcher,
    store: SharedTokenStore,
}

impl TokenService {
    pub fn new(token_store: SharedTokenStore) -> Self {
        // let root_resolver = RootResolver::get().unwrap();

        let token_fetcher = Arc::new(TokenFetcherService::default()) as SharedTokenFetcher;
        // let token_store = root_resolver.persistence.clone() as SharedTokenStore;
        TokenService {
            fetcher: token_fetcher,
            store: token_store,
        }
    }
}
