use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "Besui CLI")]
#[clap(author = "Quang Ng <quangkeu95@gmail.com>")]
#[clap(version = "0.0.1")]
#[clap(about = "Besui interactive commands", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {

}
