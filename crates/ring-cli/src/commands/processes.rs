use clap::Command;
use crossterm::style::Stylize;
use ring_cli_tree::Tree;
use sysinfo::{ProcessRefreshKind, RefreshKind, System, Users};
use tracing::{instrument, trace};
use ring_cli_list::List;
use ring_core::Core;

/// Prepare processes command parsing
pub fn setup() -> Command {
    Command::new("processes")
        .visible_alias("ps")
        .about("List running processes")
}

/// Handle processes command execution
#[instrument(name = "cli.processes", skip_all)]
pub fn handle(core: &Core) -> anyhow::Result<()> {
    // List processes
    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );
    let users = Users::new_with_refreshed_list();

    // Build tree
    let mut tree = Tree::new();

    for (&pid, process) in sys.processes() {
        if let Some(parent) = process.parent() {
            tree.add_node(pid, parent);
        } else {
            tree.add_root(pid);
        }
    }

    // Print processes
    let list: List = tree.iter()
        .map(|node| {
            let process = sys.process(*node.key);
            let task = process.and_then(|p| core.detect_tasks(p));

            vec![
                node.to_string(),
                process
                    .and_then(|p| p.user_id())
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
                process
                    .map(|p| p.name())
                    .and_then(|c| c.to_str())
                    .map(|str| str.to_string())
                    .unwrap_or("unknown".dark_grey().to_string()),
            ]
        })
        .collect();

    print!("{}", list);

    Ok(())
}