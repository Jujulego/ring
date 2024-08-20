use std::env;
use anyhow::Context;
use clap::Command;
use itertools::Itertools;
use tracing::warn;
use ring_cli_formatters::ListFormatter;
use ring_core::RingCore;

pub fn build_command() -> Command {
    Command::new("current")
        .visible_alias("pwd")
}

pub fn handle_command(core: &RingCore) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.canonicalize()
        .with_context(|| format!("Unable to access {}", current_dir.display()))?;

    let detector = core.project_detector();
    let mut list = ListFormatter::new();
    
    for project in detector.detect_from(&current_dir) {
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