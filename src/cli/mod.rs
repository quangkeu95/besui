use anyhow::Result;
use clap::Parser;
use tracing::info;

use crate::server::http_server::run_http_server;

#[derive(Debug, Parser)]
#[clap(
    name = "besui",
    about = "Multichain triggers and actions workflow",
    rename_all = "kebab-case",
    author = "Quang Ng <quangkeu95@gmail.com>",
    version
)]
#[clap()]
pub enum Cli {
    /// Start the server
    #[clap(name = "start")]
    Start {},
    /// Watch transfer on chain
    #[clap(name = "watch-transfer-on-chain")]
    WatchTransferOnChain {
        #[clap(long, value_parser, help = "Chain ID")]
        chain_id: u64,
        #[clap(long, value_parser, help = "Token address")]
        token_address: String,
        #[clap(long, value_parser, help = " Wallet address")]
        wallet_address: String,
    },
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self {
            Cli::Start {} => {
                let _ = run_http_server().await;
            }
            Cli::WatchTransferOnChain {
                chain_id,
                token_address,
                wallet_address,
            } => {
                info!(
                    chain_id = chain_id,
                    token_address = token_address,
                    wallet_address = wallet_address
                );
            }
        }
        Ok(())
    }
}
