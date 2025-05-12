use clap::{arg, value_parser, ArgMatches, Command};
use ring_core::Core;
use sysinfo::Pid;
use tracing::instrument;

/// Prepare tasks inspect command parsing
pub fn setup() -> Command {
    Command::new("inspect")
        .about("Inspect running tasks")
        .args([
            arg!(<pid> "Pid of the task to inspect")
                .value_parser(value_parser!(Pid)),
        ])
}

/// Handle tasks inspect command execution
#[instrument(name = "cli.tasks.inspect", skip_all)]
pub fn handle(core: &Core, args: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}