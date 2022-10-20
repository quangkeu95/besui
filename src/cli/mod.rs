use anyhow::Result;
use clap::{Parser, Subcommand};
use prettytable::{row, Table};
use tracing::{error, info};

use crate::{
    common::output::TableOutput, server::http_server::run_http_server, token::get_list_token_ids,
};

#[derive(Debug, Parser)]
#[clap(
    name = "besui",
    about = "Multichain triggers and actions workflow",
    rename_all = "kebab-case",
    author = "Quang Ng <quangkeu95@gmail.com>",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the server
    #[clap(name = "start")]
    Start {},
    /// Watch transfer on chain
    #[clap(name = "watch-transfer-on-chain")]
    WatchTransferOnChain {
        #[arg(long, value_parser, help = "Chain ID")]
        chain_id: u64,
        #[arg(long, value_parser, help = "Token address")]
        token_address: String,
        #[arg(long, value_parser, help = " Wallet address")]
        wallet_address: String,
    },
    /// Get specific token info
    #[clap(name = "token")]
    Token {
        #[arg(
            long,
            value_parser,
            help = "Token id (can obtains from token ids command)"
        )]
        token_id: Option<String>,

        #[command(subcommand)]
        command: TokenCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum TokenCommands {
    #[clap(name = "list-ids")]
    ListIds,
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match &self.command {
            Commands::Start {} => {
                let _ = run_http_server().await;
            }
            Commands::WatchTransferOnChain {
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
            Commands::Token { token_id, command } => match command {
                TokenCommands::ListIds => {
                    match get_list_token_ids().await {
                        Ok(list_tokens) => {
                            let mut table = TableOutput::new(vec!["ID", "Symbol", "Name"]);
                            table.add_rows(
                                list_tokens
                                    .into_iter()
                                    .map(move |item| vec![item.id, item.symbol, item.name])
                                    .collect::<Vec<Vec<String>>>(),
                            );

                            table.printstd();
                        }
                        Err(err) => error!("get list token ids error: {}", err),
                    };
                }
            },
        }
        Ok(())
    }
}
