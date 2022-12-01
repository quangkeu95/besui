use anyhow::{self, Context};
use besui_api::router::ApiController;
use besui_config::AppConfig;
use besui_core::core::BesuiCore;
use std::sync::Arc;
// use besui_core::database::DbConnectionManager;
use clap::{Parser, Subcommand};

pub mod telemetry;

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
    /// Start the http server
    #[clap(name = "start")]
    Start {},
}

impl Cli {
    pub async fn execute(self, config: &AppConfig) -> anyhow::Result<()> {
        match &self.command {
            Commands::Start {} => {
                let mut core = BesuiCore::new(config).await?;

                core.start().await.context("cannot start core")?;

                let _ = ApiController::serve(config.http_server.port, Arc::new(core))
                    .await
                    .context("could not initialize http server");
            }
        }
        Ok(())
    }
}
