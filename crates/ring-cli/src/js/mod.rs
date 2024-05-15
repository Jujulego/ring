use anyhow::Result;
use clap::{ArgMatches, Command};

mod list;

pub fn build_command() -> Command {
    Command::new("js")
        .subcommand(list::build_command())
}

pub fn handle_command(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("list", matches)) => list::handle_command(matches),
        _ => unreachable!()
    }
}