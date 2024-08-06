use std::env;
use anyhow::Context;
use clap::Command;
use tracing::warn;
use ring_js::JsProjectDetector;
use ring_rust::RustProjectDetector;
use ring_traits::ProjectDetector;

pub fn build_command() -> Command {
    Command::new("current")
        .visible_alias("pwd")
}

pub fn handle_command() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.canonicalize()
        .with_context(|| format!("Unable to access {}", current_dir.display()))?;

    let detectors: [&dyn ProjectDetector; 2] = [
        &JsProjectDetector::new(),
        &RustProjectDetector::new()
    ];

    let mut found = false;
    
    for detector in detectors {
        if let Some(project) = detector.detect_from(&current_dir)? {
            println!("{}", project.name());
            found = true;
        }
    }
    
    if !found {
        warn!("No matching project found");
    }

    Ok(())
}