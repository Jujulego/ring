mod current;
mod list;

use clap::{ArgMatches, Command};

pub fn build_command() -> Command {
    Command::new("projects")
        .aliases(["project"])
        .visible_aliases(["prj"])
        .subcommand_required(true)
        .subcommands([
            current::build_command(),
            list::build_command(),
        ])
}

pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("current", _)) => current::handle_command(),
        Some(("list", args)) => list::handle_command(args),
        _ => unreachable!()
    }
}