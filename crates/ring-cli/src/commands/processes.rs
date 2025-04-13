use clap::Command;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::{instrument, trace};
use ring_cli_list::List;

/// Prepare processes command parsing
pub fn setup() -> Command {
    Command::new("processes")
        .visible_alias("ps")
        .about("List running processes")
}

/// Handle processes command execution
#[instrument(name = "cli.processes", skip_all)]
pub fn handle() -> anyhow::Result<()> {
    // List processes
    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    let list = sys.processes().iter()
        .map(|(pid, process)| vec![
            pid.to_string(),
            process.cmd().first()
                .and_then(|c| c.to_str())
                .unwrap_or("unknown")
                .to_string()
        ])
        .collect::<List>();

    print!("{}", list);

    Ok(())
}