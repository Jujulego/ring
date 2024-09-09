mod current;
mod list;
mod tree;

use clap::{ArgMatches, Command};
use ring_core::RingCore;

pub fn build_command() -> Command {
    Command::new("projects")
        .aliases(["prj"])
        .visible_aliases(["project"])
        .subcommand_required(true)
        .subcommands([
            current::build_command(),
            list::build_command(),
            tree::build_command(),
        ])
}

pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("current", _)) => current::handle_command(core),
        Some(("list", args)) => list::handle_command(core, args),
        Some(("tree", args)) => tree::handle_command(core, args),
        _ => unreachable!()
    }
}