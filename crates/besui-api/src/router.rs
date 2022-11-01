use crate::{tokens::TokensRouter, utils::shutdown_signal};
use anyhow::Context;
use axum::{routing::get, Extension, Json, Router, Server};
use besui_core::core::BesuiCore;
use besui_types::PingResponse;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct ApiController;

impl ApiController {
    pub async fn serve(port: u16, app_core: Arc<BesuiCore>) -> anyhow::Result<()> {
        let app = Router::new()
            .nest(
                "/api",
                Router::new()
                    .merge(TokensRouter::routes())
                    .route("/api/ping", get(Self::ping)),
            )
            .layer(TraceLayer::new_for_http())
            .layer(Extension(app_core));

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!(
            "All routes initialized, HTTP server listening on http://{}",
            addr
        );

        Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("error while starting HTTP server")?;

        Ok(())
    }

    async fn ping() -> Json<PingResponse> {
        info!("received ping request");
        Json(PingResponse::default())
    }
}
