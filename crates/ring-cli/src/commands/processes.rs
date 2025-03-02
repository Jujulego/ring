use std::cell::RefCell;
use crate::core::RingCore;
use bytesize::ByteSize;
use clap::{arg, ArgAction, ArgMatches, Command};
use crossterm::style::{self, ContentStyle, Stylize};
use itertools::Itertools;
use ring_cli_table::CliTable;
use ring_tag::Tag;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use sysinfo::{Pid, Process, ProcessRefreshKind, RefreshKind, System};
use tracing::{debug, instrument, trace};
use ring_core::ProcessUnit;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub fn build_command() -> Command {
    Command::new("processes").visible_alias("ps")
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

#[instrument(name = "cli.processes", skip(core, args))]
pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List processes
    trace!("load running processes");
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything()),
    );

    // Build process unit tree
    let (tree, orphans) = build_process_tree(sys.processes());
    let mut stack: VecDeque<(Pid, Option<Rc<RefCell<ProcessUnitTree>>>)> = VecDeque::from_iter(orphans.iter().map(|&pid| (pid, None)));

    let mut roots: BTreeMap<Pid, Rc<RefCell<ProcessUnitTree>>> = BTreeMap::new();

    while let Some((pid, mut parent)) = stack.pop_front() {
        // Detect process
        if let Some(process) = sys.process(pid) {
            for detector in core.process_unit_detectors() {
                if let Some(unit) = detector.detect(&pid, process)? {
                    // Remain hidden !
                    if !show_all && unit.should_hide() {
                        if let Some(name) = unit.running_code_unit().and_then(|u| u.name().map(|n| n.to_string())) {
                            debug!("hide process {pid} running on {name}");
                        } else {
                            debug!("hide process {pid}");
                        }

                        break;
                    }

                    // Build node
                    let node = Rc::new(RefCell::new(ProcessUnitTree {
                        unit,
                        process,
                        children: BTreeMap::new()
                    }));

                    if let Some(parent) = parent {
                        debug!("recognized {} as child of {}", &pid, parent.borrow().process.pid());
                        parent.borrow_mut().children.insert(pid, node.clone());
                    } else {
                        debug!("recognized {} as root", &pid);
                        roots.insert(pid, node.clone());
                    }

                    parent = Some(node);

                    break;
                }
            }
        }

        // Detect children
        for child in tree.get(&pid).unwrap() {
            stack.push_front((*child, parent.clone()));
        }
    }

    // Render as a table
    let mut stack: VecDeque<(_, Vec<bool>)> = VecDeque::from_iter(roots.values().cloned().map(|n| (n, vec![])));
    let mut table = CliTable::new();

    while let Some((node, prefix)) = stack.pop_front() {
        let node = node.borrow();

        // Print node
        table.add_styled_row(
            [
                &format!("{}{}", display_prefix(&prefix), node.process.pid()),
                &node.unit.running_code_unit()
                    .map(|u| u.parent().unwrap_or(u))
                    .and_then(|u| u.name()
                        .map(|n| n.to_string().stylize()))
                    .unwrap_or("unknown".to_string().dark_grey()),
                &format!("{:>10}", ByteSize::b(node.process.memory())),
                &node.unit.tags().iter().map(Tag::stylize).join(" "),
                &node.unit.cmd().join(" "),
            ],
            if node.unit.should_hide() {
                ContentStyle::new().attribute(style::Attribute::Dim)
            } else {
                Default::default()
            }
        );

        // Print its children
        for (idx, child) in node.children.values().enumerate().rev() {
            let mut child_prefix = prefix.clone();
            child_prefix.push(idx >= node.children.len() - 1);

            stack.push_front((child.clone(), child_prefix));
        }
    }

    for row in &table {
        println!("{row}");
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// Process tree
////////////////////////////////////////////////////////////////////////////////

fn build_process_tree(processes: &HashMap<Pid, Process>) -> (HashMap<Pid, HashSet<Pid>>, HashSet<Pid>) {
    let mut tree = HashMap::new();
    let mut orphans = HashSet::new();

    for (&pid, process) in processes {
        tree.entry(pid).or_insert_with(HashSet::new);

        if let Some(parent) = process.parent() {
            orphans.remove(&pid);


            tree.entry(parent)
                .or_insert_with(|| {
                    orphans.insert(parent);
                    HashSet::new()
                })
                .insert(pid);
        } else {
            orphans.insert(pid);
        }
    }

    (tree, orphans)
}

////////////////////////////////////////////////////////////////////////////////
// Unit tree
////////////////////////////////////////////////////////////////////////////////

struct ProcessUnitTree<'a> {
    unit: Rc<dyn ProcessUnit>,
    process: &'a Process,
    children: BTreeMap<Pid, Rc<RefCell<ProcessUnitTree<'a>>>>,
}

fn display_prefix(prefix: &[bool]) -> String {
    if prefix.is_empty() {
        return "\u{25CF} ".to_string();
    }

    let mut result = String::with_capacity(prefix.len() * 2);

    for &is_last in prefix[..prefix.len() - 1].iter() {
        if is_last {
            result.push_str("  ");
        } else {
            result.push_str(" \u{2502}");
        }
    }

    if prefix[prefix.len() - 1] {
        result.push_str("\u{2514}\u{2574}");
    } else {
        result.push_str("\u{251C}\u{2574}");
    }

    result
}