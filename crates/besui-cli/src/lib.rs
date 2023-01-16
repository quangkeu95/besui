use anyhow::{self, Context};
use besui_api::router::ApiController;
use besui_config::AppConfig;
use besui_core::core::BesuiCore;
use std::sync::Arc;
// use besui_core::database::DbConnectionManager;
use clap::{crate_version, Command};

mod nft;
mod server;
pub mod telemetry;

// #[derive(Debug, Parser)]
// #[clap(
//     name = "besui",
//     about = "Multichain triggers and actions workflow",
//     rename_all = "kebab-case",
//     author = "Quang Ng <quangkeu95@gmail.com>",
//     version
// )]
pub struct Cli {
    // #[command(subcommand)]
    // command: Commands,
}

impl Cli {
    pub fn parse() {
        let matches = Command::new("besui")
            .author("Quang Ng <quangkeu95@gmail.com>")
            .about("Multichain trigger and actions workflow")
            .version(crate_version!())
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(server::command())
            .subcommand(nft::command())
            .get_matches();

        match matches.subcommand() {
            Some(("server", args)) => server::execute(args),
            Some(("nft", args)) => nft::execute(args),
            _ => unreachable!(),
        }
    }

    pub async fn execute(self, config: &AppConfig) -> anyhow::Result<()> {
        // match &self.command {
        //     Commands::Start {} => {
        //         let mut core = BesuiCore::new(config).await?;

        //         core.start().await.context("cannot start core")?;

        //         let _ = ApiController::serve(config.http_server.port, Arc::new(core))
        //             .await
        //             .context("could not initialize http server");
        //     }
        //     Commands::Nft => todo!(),
        // }
        Ok(())
    }
}
