use anyhow::Context;
use axum::{routing::get, Json, Router, Server};
use besui_types::PingResponse;
use std::net::SocketAddr;
use tracing::info;

pub struct ApiController;

impl ApiController {
    pub async fn serve(port: u16) -> anyhow::Result<()> {
        let app = Router::new().route("/api/ping", get(Self::ping));

        info!("routes initialized, listening on port {}", port);
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .context("error while starting API server")?;

        Ok(())
    }

    async fn ping() -> Json<PingResponse> {
        info!("received ping request");
        Json(PingResponse::default())
    }
}
