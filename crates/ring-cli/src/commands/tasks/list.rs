use crate::commands::tasks::utils::ProcessAncestors;
use bytesize::ByteSize;
use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use crossterm::style::Stylize;
use ring_cli_list::List;
use ring_cli_tree::Tree;
use ring_core::{Core, TaskCache, TaskRegistry};
use std::collections::HashSet;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System, Users};
use tracing::{instrument, trace};

/// Prepare tasks list command parsing
pub fn setup() -> Command {
    Command::new("list")
        .visible_aliases(["ls", "ps"])
        .about("List running tasks")
        .args([
            arg!([pid] "Pid to focus")
                .value_parser(value_parser!(Pid)),
            arg!(-a --all "Display all processes")
                .action(ArgAction::SetTrue)
        ])
}

/// Handle tasks list command execution
#[instrument(name = "cli.tasks.list", skip_all, fields(options.all = args.get_flag("all")))]
pub fn handle(core: &Core, args: &ArgMatches) -> anyhow::Result<()> {
    // Parse arguments
    let focused_pid = args.get_one::<Pid>("pid");
    let show_all = args.get_flag("all");

    // List processes
    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    trace!("load users");
    let users = Users::new_with_refreshed_list();

    // Build tree
    let mut tree = Tree::new();
    let tasks = TaskCache::new(core);

    let focused_ancestors = focused_pid.and_then(|&pid| sys.process(pid)).iter()
        .flat_map(|process| ProcessAncestors::new(&sys, process))
        .map(|process| process.pid())
        .collect::<HashSet<_>>();

    for (&pid, process) in sys.processes() {
        let task = tasks.detect_task(process);

        let focused = if let Some(focused_pid) = focused_pid {
            if focused_ancestors.contains(&pid) {
                true
            } else {
                ProcessAncestors::new(&sys, process)
                    .map(|process| process.pid())
                    .any(|pid| &pid == focused_pid)
            }
        } else {
            true
        };

        if focused && (task.is_some() || show_all) {
            let ancestors = ProcessAncestors::new(&sys, process);

            let parent = ancestors.skip(1)
                .find(|proc| show_all || tasks.detect_task(proc).is_some());

            if let Some(parent) = parent {
                tree.add_node(pid, parent.pid());
            } else {
                tree.add_root(pid);
            }
        }
    }

    // Print processes
    let list: List = tree.iter()
        .map(|node| {
            let process = sys.process(*node.key).unwrap();
            let task = tasks.detect_task(process);

            let mut line = vec![
                node.to_string(),
                process.user_id()
                    .and_then(|uid| users.get_user_by_id(uid))
                    .map(|u| u.name().to_string())
                    .unwrap_or("unknown".dark_grey().to_string()),
                task.as_ref()
                    .map(|t| t.style().apply(t.kind()).to_string())
                    .unwrap_or("unknown".dark_grey().to_string()),
                task.as_ref()
                    .and_then(|t| t.working_unit())
                    .and_then(|u| u.name().map(|s| u.style().apply(s).to_string()))
                    .unwrap_or("unknown".dark_grey().to_string()),
                format!("{:>10}", ByteSize::b(process.virtual_memory())),
                task.and_then(|t| t.exe().file_name().and_then(|s| s.to_str()).map(|s| s.to_string()))
                    .or_else(|| process.name().to_str().map(|s| s.to_string()))
                    .unwrap_or("unknown".dark_grey().to_string()),
            ];

            if focused_pid.is_some_and(|pid| pid == node.key) {
                line.iter_mut()
                    .for_each(|item| *item = item.clone().bold().to_string());
            }

            line
        })
        .collect();

    print!("{list}");

    Ok(())
}