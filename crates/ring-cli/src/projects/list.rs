use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use anyhow::Context;
use clap::{arg, ArgMatches, Command, value_parser};
use tracing::warn;
use ring_js::{JsProjectDetector, JsScopeDetector};
use ring_traits::{Project, Scope, ScopeDetector};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
}

pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let path = current_dir.join(path).canonicalize()
        .with_context(|| format!("Unable to access {}", path.display()))?;

    let detector = JsScopeDetector::new(Rc::new(JsProjectDetector::new()));

    if let Some(scope) = detector.detect_from(&path)? {
        for project in scope.projects() {
            let project = project?;
            
            println!("{}", project.name());
        }
    } else {
        warn!("No matching scope found");
    }

    Ok(())
}