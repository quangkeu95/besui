use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{debug, info};

use crate::server::http_server::run_http_server;

#[derive(Debug, Parser)]
#[clap(
    name = "besui",
    about = "Multichain triggers and actions workflow",
    rename_all = "kebab-case",
    author = "Quang Ng <quangkeu95@gmail.com>",
    version
)]
pub enum Cli {
    /// Start the server
    #[clap(name = "start")]
    Start {},
    /// Binance interactive commands
    #[clap(name = "binance")]
    Binance {
        #[clap(long = "apiKey", value_parser, help = "Binance API key")]
        api_key: Option<String>,
        #[clap(long = "apiSecret", value_parser, help = "Binance API secret")]
        api_secret: Option<String>,
        #[clap(subcommand)]
        command: BinanceCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum BinanceCommands {}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self {
            Cli::Start {} => {
                let _ = run_http_server().await;
            }
            Cli::Binance {
                api_key,
                api_secret,
                command,
            } => {
                debug!(binance_api_key = api_key, binance_api_secret = api_secret);
            }
        }
        Ok(())
    }
}
