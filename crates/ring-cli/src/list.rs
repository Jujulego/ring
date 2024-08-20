use std::collections::BTreeSet;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use std::rc::Rc;
use anyhow::Context;
use clap::{arg, ArgAction, ArgMatches, Command, value_parser};
use itertools::Itertools;
use lscolors::LsColors;
use owo_colors::colors::BrightBlack;
use owo_colors::OwoColorize;
use tracing::info;
use ring_cli_formatters::ListFormatter;
use ring_core::CombinedDetector;
use ring_js::{JsProjectDetector, JsScopeDetector};
use ring_rust::{RustProjectDetector, RustScopeDetector};
use ring_traits::Tagged;
use ring_utils::Tag;

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
    let js_scope_detector = Rc::new(JsScopeDetector::new(js_project_detector.clone()));
    let rust_project_detector = Rc::new(RustProjectDetector::new());
    let rust_scope_detector = Rc::new(RustScopeDetector::new(rust_project_detector.clone()));

    let detector: CombinedDetector<Rc<dyn Tagged>> = CombinedDetector::new(vec![
        js_project_detector,
        js_scope_detector,
        rust_project_detector,
        rust_scope_detector,
    ]);

    let colors = LsColors::from_env().unwrap_or_default();
    let mut list = ListFormatter::new();

    if path.is_dir() {
        info!("Searching project root from {}", path.display());
        
        for entry in read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_str().unwrap().to_string();
            
            if !show_all && file_name.starts_with('.') {
                continue;
            }

            let file_style = colors.style_for(&entry)
                .map(lscolors::Style::to_owo_colors_style)
                .unwrap_or_default();

            let mut tags: BTreeSet<&'static Tag> = BTreeSet::new();

            for project in detector.detect_from(&entry.path()) {
                match project {
                    Ok(project) => tags.extend(project.tags()),
                    Err(err) => return Err(err),
                }
            }

            if !tags.is_empty() {
                list.add_row([
                    &tags.iter().join("/"),
                    &file_name.style(file_style),
                ]);
            } else {
                list.add_row([&"none".bright_black(), &file_name.style(file_style)]);
            }
        }
    } else {
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap();
        let mut tags: BTreeSet<&'static Tag> = BTreeSet::new();

        for project in detector.detect_from(&path) {
            match project {
                Ok(project) => tags.extend(project.tags()),
                Err(err) => return Err(err),
            }
        }

        if !tags.is_empty() {
            list.add_row([
                &tags.iter().join("/"),
                &file_name,
            ]);
        } else {
            list.add_row([&"none".fg::<BrightBlack>(), &file_name]);
        }
    }
    
    println!("{list}");

    Ok(())
}
