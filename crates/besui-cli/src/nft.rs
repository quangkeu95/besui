use clap::{Arg, ArgMatches, Command};
use tracing::info;

pub fn command() -> impl Into<Command> {
    Command::new("nft")
        .about("Scan and query NFT collections.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(nft_scan_command())
}

pub fn nft_scan_command() -> impl Into<Command> {
    Command::new("scan")
        .about("Scan list of NFT collections which is presented on Opensea.")
        .arg(
            Arg::new("from-timestamp")
                .help("From timestamp (in seconds)")
                .required(true),
        )
        .arg(
            Arg::new("to-timestamp")
                .help("To timestamp (in seconds)")
                .required(true),
        )
}

pub fn execute(args: &ArgMatches) {
    match args.subcommand() {
        Some(("scan", scan_args)) => {
            info!("Running NFT collection scan...");
        }
        _ => unreachable!(),
    }
}
