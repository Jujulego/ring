use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use anyhow::Context;
use clap::{arg, ArgMatches, Command, value_parser};
use tracing::warn;
use ring_cli_formatters::ListFormatter;
use ring_js::{JsProjectDetector, JsScopeDetector};
use ring_rust::{RustProjectDetector, RustScopeDetector};
use ring_traits::ScopeDetector;

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

    let detectors: [&ScopeDetector; 2] = [
        &JsScopeDetector::new(Rc::new(JsProjectDetector::new())),
        &RustScopeDetector::new(Rc::new(RustProjectDetector::new()))
    ];

    let mut list = ListFormatter::new();

    for detector in detectors {
        if let Some(scope) = detector.detect_from(&path).into_result()? {
            for project in scope.projects() {
                let project = project?;
                
                list.add_row([&project.name(), &project.tags().join(", ")]);
            }
        }
    }

    if !list.is_empty() {
        println!("{list}");
    } else {
        warn!("No matching project found");
    }

    Ok(())
}