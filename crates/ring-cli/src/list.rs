use std::collections::BTreeSet;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use std::rc::Rc;
use anyhow::Context;
use clap::{arg, ArgAction, ArgMatches, Command, value_parser};
use colored::Colorize;
use tracing::info;
use ring_cli_formatters::ListFormatter;
use ring_js::{JsProjectDetector, JsScopeDetector};
use ring_rust::{RustProjectDetector, RustScopeDetector};
use ring_traits::TaggedDetector;
use ring_utils::OptionalResult::{Empty, Fail, Found};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let path = current_dir.join(path).canonicalize()
        .with_context(|| format!("Unable to access {}", path.display()))?;

    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List directory files
    let js_project_detector = Rc::new(JsProjectDetector::new());
    let rust_project_detector = Rc::new(RustProjectDetector::new());

    let detectors: [&TaggedDetector; 4] = [
        js_project_detector.as_ref(),
        &JsScopeDetector::new(js_project_detector.clone()),
        rust_project_detector.as_ref(),
        &RustScopeDetector::new(rust_project_detector.clone())
    ];

    let mut list = ListFormatter::new();

    if path.is_dir() {
        info!("Searching project root from {}", path.display());
        
        for entry in read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_str().unwrap().to_string();
            
            if !show_all && file_name.starts_with('.') {
                continue;
            }
            
            let file_name = {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        file_name.blue()
                    } else if file_type.is_symlink() {
                        file_name.cyan()
                    } else {
                        file_name.normal()
                    }
                } else {
                    file_name.normal()
                }
            };

            let mut tags = BTreeSet::new();

            for detector in detectors {
                match detector.detect_from_as(&entry.path()) {
                    Found(project) => tags.extend(project.tags()),
                    Fail(err) => return Err(err),
                    Empty => continue,
                }
            }

            if !tags.is_empty() {
                list.add_row([
                    &tags.iter().copied().collect::<Vec<&str>>().join("/"),
                    &file_name,
                ]);
            } else {
                list.add_row([&"none".bright_black(), &file_name]);
            }
        }
    } else {
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap();
        let mut tags: BTreeSet<&str> = BTreeSet::new();

        for detector in detectors {
            match detector.detect_from_as(&path) {
                Found(project) => tags.extend(project.tags()),
                Fail(err) => return Err(err),
                Empty => continue,
            }
        }

        if !tags.is_empty() {
            list.add_row([
                &tags.iter().copied().collect::<Vec<&str>>().join("/"),
                &file_name,
            ]);
        } else {
            list.add_row([&"none".bright_black(), &file_name]);
        }
    }
    
    println!("{list}");

    Ok(())
}
