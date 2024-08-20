use std::collections::BTreeSet;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use anyhow::Context;
use clap::{arg, ArgAction, ArgMatches, Command, value_parser};
use itertools::Itertools;
use lscolors::LsColors;
use owo_colors::colors::BrightBlack;
use owo_colors::OwoColorize;
use tracing::info;
use ring_cli_formatters::ListFormatter;
use ring_core::RingCore;
use ring_utils::Tag;

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let path = current_dir.join(path).canonicalize()
        .with_context(|| format!("Unable to access {}", path.display()))?;

    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List directory files
    let detector = core.tagged_detector();

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
