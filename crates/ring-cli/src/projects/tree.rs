use std::collections::VecDeque;
use std::env;
use std::path::PathBuf;
use clap::{arg, value_parser, ArgMatches, Command};
use itertools::Itertools;
use tracing::warn;
use ring_cli_formatters::ListFormatter;
use ring_core::RingCore;
use ring_utils::Normalize;

pub fn build_command() -> Command {
    Command::new("tree")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
}

pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?.normalize();
    let path = args.get_one::<PathBuf>("path")
        .map(|path| path.resolve(&current_dir))
        .unwrap_or(current_dir);

    let detector = core.project_detector();
    let mut projects = detector.detect_from(&path)
        .collect::<anyhow::Result<VecDeque<_>>>()?;

    if projects.is_empty() {
        warn!("No matching project found");
        return Ok(());
    }

    // Print current project as root
    let mut list = ListFormatter::new();

    for project in &projects {
        list.add_row([
            &project.name(),
            &project.tags().iter().join("/")
        ]);
    }

    println!("{list}");

    // Recursively print their deps
    while let Some(prj) = projects.pop_front() {
        for dep in prj.dependencies() {
            println!("+ {dep}");
        }
    }

    Ok(())
}