use clap::{ArgMatches, Command};

pub fn command() -> impl Into<Command> {
    Command::new("server").about("Run server")
}

pub fn execute(args: &ArgMatches) {}
