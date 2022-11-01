use anyhow;
use besui_cli::Cli;
use besui_config::config::AppConfig;
use besui_core::database::DbConnectionManager;
use clap::Parser;
use dotenvy::dotenv;
use std::sync::Arc;
use tokio;
use tracing::info;
use tracing_subscriber::{self, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(AppConfig::parse().unwrap());

    info!("Starting besui...");
    let cli = Cli::parse();

    cli.execute(config).await?;

    Ok(())
}
