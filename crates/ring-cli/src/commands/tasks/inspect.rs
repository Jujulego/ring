use anyhow::anyhow;
use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use ring_core::{Core, ProcessTaskRegistry};
use std::io::{stdout, IsTerminal};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tracing::{instrument, trace, warn};

/// Prepare tasks inspect command parsing
pub fn setup() -> Command {
    Command::new("inspect")
        .about("Inspect running tasks")
        .args([
            arg!(<pid> "Pid of the task to inspect")
                .value_parser(value_parser!(Pid)),
            arg!(--pretty "Force pretty print")
                .action(ArgAction::SetTrue),
        ])
}

/// Handle tasks inspect command execution
#[instrument(name = "cli.tasks.inspect", skip_all)]
pub fn handle(core: &Core, args: &ArgMatches) -> anyhow::Result<()> {
    // Parse arguments
    let pid = args.get_one::<Pid>("pid").unwrap();

    // Load process
    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    let process = sys.process(*pid)
        .ok_or(anyhow!("Process {pid} not found"))?;

    // Inspect process
    let task = core.detect_task(process)
        .map(|task| task.inspect())
        .ok_or(anyhow!("Process {pid} not recognized"))?;

    if stdout().is_terminal() || args.get_flag("pretty") {
        serde_json::to_writer_pretty(stdout(), &task)?;
    } else {
        serde_json::to_writer(stdout(), &task)?;
    }

    Ok(())
}