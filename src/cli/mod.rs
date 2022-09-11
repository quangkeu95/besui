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
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self {
            Cli::Start {} => {
                let _ = run_http_server().await;
            }
        }
        Ok(())
    }
}
