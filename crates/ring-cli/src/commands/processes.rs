use bytesize::ByteSize;
use clap::{arg, ArgAction, ArgMatches, Command};
use itertools::Itertools;
use owo_colors::{colors, Effect, OwoColorize, Style};
use ring_cli_table::CliTable;
use ring_process_unit::ProcessUnitDetector;
use ring_web::{WebProcessDetector, WebUnitDetector};
use std::rc::Rc;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::{debug, instrument, trace};

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

    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    for (pid, process) in sys.processes() {
        for detector in detectors {
            if let Some(unit) = detector.detect(pid, process)? {
                let mut style = Style::new();

                if unit.should_hide() {
                    if !show_all {
                        if let Some(name) = unit.running_code_unit().and_then(|u| u.name().map(|n| n.to_string())) {
                            debug!("hide process {pid} running on {name}");
                        } else {
                            debug!("hide process {pid}");
                        }

                        break;
                    } else {
                        style = style.effect(Effect::Dimmed)
                    }
                }

                table.add_row([
                    &format!("{:>5}", pid.as_u32()).style(style),
                    &unit.running_code_unit()
                        .map(|u| u.parent().unwrap_or(u))
                        .and_then(|u| u.name()
                            .map(|n| n.to_string()))
                        .unwrap_or("unknown".to_string().fg::<colors::BrightBlack>().to_string())
                        .style(style),
                    &format!("{:>9}", ByteSize::b(process.memory())).style(style),
                    &unit.tags().iter().map(|tag| tag.styled()).join(" ").style(style),
                    &unit.cmd().join(" ").style(style),
                ]);

                break;
            }
        }
    }

    table.sort_by(|a, b| a.get(0).cmp(&b.get(0)));

    for row in &table {
        println!("{row}");
    }

    Ok(())
}
