use std::env;
use clap::Command;
use itertools::Itertools;
use tracing::warn;
use ring_cli_formatters::ListFormatter;
use ring_core::RingCore;
use ring_utils::Normalize;

pub fn build_command() -> Command {
    Command::new("current")
        .visible_alias("pwd")
}

pub fn handle_command(core: &RingCore) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?.normalize();

    let detector = core.project_detector();
    let mut list = ListFormatter::new();

    // TODO: pass a normalized path to detector
    for project in detector.detect_from(current_dir.as_ref()) {
        let project = project?;
        
        list.add_row([
            &project.name(),
            &project.tags().iter().join("/")
        ]);
    }

    if !list.is_empty() {
        println!("{list}");
    } else {
        warn!("No matching project found");
    }

    Ok(())
}