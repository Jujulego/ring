use std::env;
use std::path::PathBuf;
use clap::{arg, ArgMatches, Command, value_parser};
use itertools::Itertools;
use tracing::warn;
use ring_cli_formatters::ListFormatter;
use ring_core::RingCore;
use ring_utils::Normalize;

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
}

pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?.normalize();
    let path = args.get_one::<PathBuf>("path")
        .map(|path| path.resolve(&current_dir))
        .unwrap_or(current_dir);

    let detector = core.scope_detector();
    let mut list = ListFormatter::new();

    for scope in detector.detect_from(&path) {
        let scope = scope?;
        
        for project in scope.projects() {
            let project = project?;

            list.add_row([
                &project.name(),
                &project.tags().iter().join("/")
            ]);
        }
    }

    if !list.is_empty() {
        println!("{list}");
    } else {
        warn!("No matching scope found");
    }

    Ok(())
}