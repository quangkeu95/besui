use anyhow;
use besui_cli::{telemetry, Cli};
use besui_config::APP_CONFIG_INSTANCE;
use clap::Parser;
use dotenvy::dotenv;
use std::str::FromStr;
use std::sync::Arc;
use tokio;
use tracing::{info, log::LevelFilter};
use tracing_subscriber::{self, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = &APP_CONFIG_INSTANCE;
    let crate_log_level =
        LevelFilter::from_str(config.log_level.as_str()).unwrap_or_else(|_| LevelFilter::Info);
    let http_log_level = LevelFilter::from_str(config.http_server.log_level.as_str())
        .unwrap_or_else(|_| LevelFilter::Off);

    let tracing_options = telemetry::TracingOptionsBuilder::default()
        .crate_level(crate_log_level)
        .tower_http_level(http_log_level)
        .build()?;

    // initialize tracing
    telemetry::init_tracing("besui".into(), tracing_options);

    // info!("Starting besui...");
    let _ = Cli::parse();

    Ok(())
}
