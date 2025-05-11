use clap::{ArgMatches, Command};
use ring_core::Core;

pub mod list;
mod utils;

/// Prepare tasks command parsing
pub fn setup() -> Command {
    Command::new("tasks")
        .alias("task")
        .about("Managing running tasks")
        .subcommands([
            list::setup()
        ])
}

/// Handle tasks command execution
pub fn handle(core: &Core, args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("list", args)) => list::handle(core, args),
        None => list::handle(core, args),
        _ => unreachable!(),
    }
}