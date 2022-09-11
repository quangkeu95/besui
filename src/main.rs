use besui::cli::Cli;
use besui::config::get_global_config;
use clap::Parser;
use tokio::task;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    cli.execute().await
}
