use clap::Command;
use ring_cli_tree::Tree;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tracing::{instrument, trace};

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

    let mut tree = Tree::new();

    for (pid, process) in sys.processes() {
        if let Some(parent) = &process.parent() {
            tree.add_node(pid.to_string(), parent.to_string());
        } else {
            tree.add_root(pid.to_string());
        }
    }

    for node in &tree {
        println!("{}", node);
    }
    
    /*let list = sys.processes().iter()
        .map(|(pid, process)| vec![
            pid.to_string(),
            process.cmd().first()
                .and_then(|c| c.to_str())
                .unwrap_or("unknown")
                .to_string()
        ])
        .collect::<List>();

    print!("{}", list);*/

    Ok(())
}