use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router, Server};

use crate::config::get_global_config;
use tracing::info;

pub async fn run_http_server() -> Result<()> {
    let config = get_global_config();
    let app = Router::new().route("/ping", get(ping));

    let port = config.http_server.port as u16;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Start serving HTTP server on port {}", port);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn ping() -> &'static str {
    "pong"
}
