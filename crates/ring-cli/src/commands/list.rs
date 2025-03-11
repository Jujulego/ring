use std::env;
use std::path::PathBuf;
use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use tracing::{debug, instrument};

/// Prepare list command parsing
pub fn setup() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .aliases(["l", "ll"])
        .about("List files and directories (in current directory by default)")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all "Display all files")
            .action(ArgAction::SetTrue))
}

/// Handle list command execution
#[instrument(name = "cli.list", skip_all)]
pub fn handle(args: &ArgMatches) -> anyhow::Result<()> {
    // Extract arguments
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let show_all = args.get_flag("all");
    debug!(message = "detected options", all = show_all);

    Ok(())
}