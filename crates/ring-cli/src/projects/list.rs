use std::env;
use std::rc::Rc;
use anyhow::Context;
use clap::Command;
use tracing::warn;
use ring_js::{JsProjectDetector, JsScopeDetector};
use ring_traits::{Project, Scope, ScopeDetector};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
}

pub fn handle_command() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.canonicalize()
        .context(format!("Unable to access {}", current_dir.display()))?;

    let detector = JsScopeDetector::new(Rc::new(JsProjectDetector::new()));

    if let Some(scope) = detector.detect_from(&current_dir)? {
        for project in scope.projects() {
            let project = project?;
            
            println!("{}", project.name());
        }
    } else {
        warn!("No matching scope found");
    }

    Ok(())
}