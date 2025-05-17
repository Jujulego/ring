use clap::{ArgMatches, Command};
use ring_core::Core;
use crate::commands::tasks;

/// Prepare processes command parsing
pub fn setup() -> Command {
    tasks::list::setup()
        .name("processes")
        .visible_alias(None) // <= removes existing aliases
        .visible_alias("ps")
        .hide(true)
}

/// Handle tasks command execution
pub fn handle(core: &Core, args: &ArgMatches) -> anyhow::Result<()> {
    tasks::list::handle(core, args)
}