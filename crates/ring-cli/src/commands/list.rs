use anyhow::Context;
use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use itertools::Itertools;
use lscolors::LsColors;
use owo_colors::OwoColorize;
use ring_cli_table::CliTable;
use ring_code_unit::CodeUnitDetector;
use ring_tag::Tag;
use ring_web::WebUnitDetector;
use std::collections::BTreeSet;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use tracing::{instrument, trace};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

#[instrument(name = "cli.list", skip(args))]
pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    // Extract arguments
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let show_all = args.get_one::<bool>("all").unwrap_or(&false);
    
    // Initiate detectors
    let detectors: &[Box<dyn CodeUnitDetector>] = &[
        Box::new(WebUnitDetector::default()),
    ];

    // Test files
    let ls_colors = LsColors::from_env().unwrap_or_default();
    let mut table = CliTable::new();

    for path in list_files(path)? {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        
        if !show_all && file_name.starts_with(".") {
            continue;
        }

        let mut languages = BTreeSet::new();
        let mut tags = BTreeSet::new();

        for detector in detectors {
            if let Some(unit) = detector.detect(&path)? {
                languages.insert(Tag::from(unit.language()));
                tags.extend(unit.tags());
            }
        }

        let file_style = ls_colors.style_for_path(&path)
            .map(lscolors::Style::to_owo_colors_style)
            .unwrap_or_default();

        table.add_row([
            &file_name.if_supports_color(supports_color::Stream::Stdout, |txt| txt.style(file_style)),
            &languages.iter().map(|tag| tag.styled()).join(" "),
            &tags.iter().map(|tag| tag.styled()).join(" "),
        ]);
    }

    for row in &table {
        println!("{row}");
    }

    Ok(())
}

fn list_files(path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    if path.is_dir() {
        trace!("read directory {}", path.display());
        read_dir(path)?
            .map(|res| res
                .map(|e| e.path())
                .with_context(|| format!("Error while reading directory {:?}", path))
            )
            .collect()
    } else {
        Ok(vec![path.clone()])
    }
}