use crate::core::RingCore;
use bytesize::ByteSize;
use clap::{arg, ArgAction, ArgMatches, Command};
use crossterm::style::{self, ContentStyle, Stylize};
use itertools::Itertools;
use ring_cli_table::CliTable;
use ring_tag::Tag;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::{debug, instrument, trace};

pub fn build_command() -> Command {
    Command::new("processes").visible_alias("ps")
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

#[instrument(name = "cli.processes", skip(core, args))]
pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List processes
    let mut table = CliTable::new();

    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    for (pid, process) in sys.processes() {
        for detector in core.process_unit_detectors() {
            if let Some(unit) = detector.detect(pid, process)? {
                table.add_styled_row(
                    [
                        &format!("{:>5}", pid.as_u32()),
                        &unit.running_code_unit()
                            .map(|u| u.parent().unwrap_or(u))
                            .and_then(|u| u.name()
                                .map(|n| n.to_string().stylize()))
                            .unwrap_or("unknown".to_string().dark_grey()),
                        &format!("{:>9}", ByteSize::b(process.memory())),
                        &unit.tags().iter().map(Tag::stylize).join(" "),
                        &unit.cmd().join(" "),
                    ],
                    if unit.should_hide() {
                        if !show_all {
                            if let Some(name) = unit.running_code_unit().and_then(|u| u.name().map(|n| n.to_string())) {
                                debug!("hide process {pid} running on {name}");
                            } else {
                                debug!("hide process {pid}");
                            }

                            break;
                        } else {
                            ContentStyle::new().attribute(style::Attribute::Dim)
                        }
                    } else {
                        Default::default()
                    }
                );

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
