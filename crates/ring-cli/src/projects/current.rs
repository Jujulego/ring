use std::env;
use anyhow::Context;
use clap::Command;
use tracing::warn;
use ring_cli_formatters::ListFormatter;
use ring_js::JsProjectDetector;
use ring_rust::RustProjectDetector;
use ring_traits::ProjectDetector;
use ring_utils::OptionalResult::{Empty, Fail, Found};

pub fn build_command() -> Command {
    Command::new("current")
        .visible_alias("pwd")
}

pub fn handle_command() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.canonicalize()
        .with_context(|| format!("Unable to access {}", current_dir.display()))?;

    let detectors: [&ProjectDetector; 2] = [
        &JsProjectDetector::new(),
        &RustProjectDetector::new()
    ];

    let mut list = ListFormatter::new();
    
    for detector in detectors {
        match detector.detect_from(&current_dir) {
            Found(project) => list.add_row([&project.name(), &project.tags().join(", ")]),
            Fail(err) => return Err(err),
            Empty => continue,
        }
    }

    if !list.is_empty() {
        println!("{list}");
    } else {
        warn!("No matching project found");
    }

    Ok(())
}