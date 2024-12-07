use clap::Command;
use itertools::Itertools;
use ring_cli_table::CliTable;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::instrument;

pub fn build_command() -> Command {
    Command::new("processes").visible_alias("ps")
}

#[instrument(name = "cli.processes")]
pub fn handle_command() -> anyhow::Result<()> {
    let mut table = CliTable::new();
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    for (pid, process) in sys.processes() {
        table.add_row([
            pid,
            &process.exe()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or(""),
            &process.cwd()
                .and_then(|n| n.to_str())
                .unwrap_or(""),
            &process.cmd().iter()
                .map(|p| p.to_str().unwrap())
                .join(" ")
        ]);
    }

    for row in &table {
        println!("{row}");
    }

    Ok(())
}
