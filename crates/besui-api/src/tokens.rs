use axum::{routing::get, Extension, Router};
use besui_core::core::BesuiCore;
use std::sync::Arc;

pub(crate) struct TokensRouter {}

impl TokensRouter {
    pub fn routes() -> Router {
        Router::new().route("/tokens", get(Self::get_all_tokens))
    }

    pub async fn get_all_tokens(Extension(app_core): Extension<Arc<BesuiCore>>) {}
}
