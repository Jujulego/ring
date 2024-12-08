use bytesize::ByteSize;
use clap::{arg, ArgAction, ArgMatches, Command};
use itertools::Itertools;
use ring_cli_table::CliTable;
use ring_process_unit::ProcessUnitDetector;
use ring_web::{WebProcessDetector, WebUnitDetector};
use std::rc::Rc;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::instrument;

pub fn build_command() -> Command {
    Command::new("processes").visible_alias("ps")
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

#[instrument(name = "cli.processes", skip(args))]
pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    // Initiate detectors
    let detectors: &[Box<dyn ProcessUnitDetector>] = &[
        Box::new(WebProcessDetector::new(
            Rc::new(WebUnitDetector::default())
        ))
    ];

    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List processes
    let mut table = CliTable::new();

    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    for (pid, process) in sys.processes() {
        for detector in detectors {
            if let Some(unit) = detector.detect(pid, process)? {
                if !show_all && unit.should_hide() {
                    break;
                }

                table.add_row([
                    &format!("{:>5}", pid.as_u32()),
                    &format!("{:>5}", process.parent().map(|pid| pid.to_string()).unwrap_or_default()),
                    &format!("{:>10}", ByteSize::b(process.memory())),
                    &unit.tags().iter().map(|tag| tag.styled()).join(" "),
                    &unit.cmd().join(" "),
                ]);

                break;
            }
        }
    }

    let cols = termsize::get()
        .map(|size| size.cols)
        .unwrap_or(80) as usize;

    table.sort_by(|a, b| a.get(0).cmp(&b.get(0)));

    for row in &table {
        let line = format!("{row}");

        if line.len() > cols {
            println!("{}...", &line[..cols - 3]);
        } else {
            println!("{line}");
        }
    }

    Ok(())
}
