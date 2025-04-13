use clap::Command;
use tracing::instrument;

/// Prepare processes command parsing
pub fn setup() -> Command {
    Command::new("processes")
        .visible_alias("ps")
        .about("List running processes")
}

/// Handle processes command execution
#[instrument(name = "cli.processes", skip_all)]
pub fn handle() -> anyhow::Result<()> {
    Ok(())
}