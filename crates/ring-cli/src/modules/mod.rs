mod list;

use clap::{ArgMatches, Command};
use ring_core::RingCore;

pub fn build_command() -> Command {
    Command::new("modules")
        .aliases(["mod"])
        .subcommand_required(true)
        .subcommands([
            list::build_command()
        ])
}

pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("list", _)) => list::handle_command(core),
        _ => unreachable!()
    }
}