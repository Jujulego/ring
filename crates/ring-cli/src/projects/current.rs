use std::env;
use anyhow::Context;
use clap::Command;
use tracing::warn;
use ring_js::JsProjectDetector;
use ring_traits::{Project, ProjectDetector};

pub fn build_command() -> Command {
    Command::new("current")
        .visible_alias("pwd")
}

pub fn handle_command() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.canonicalize()
        .with_context(|| format!("Unable to access {}", current_dir.display()))?;

    let detector = JsProjectDetector::new();

    if let Some(project) = detector.detect_from(&current_dir)? {
        println!("{}", project.name());
    } else {
        warn!("No matching project found");
    }

    Ok(())
}