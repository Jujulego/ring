use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use crossterm::style::Stylize;
use itertools::Itertools;
use lscolors::LsColors;
use ring_cli_list::List;
use ring_core::traits::Location;
use ring_core::{Core, LanguageRegistry, PathRegistry, UnitRegistry};
use std::io::IsTerminal;
use std::path::PathBuf;
use std::{env, io};
use tracing::instrument;

/// Prepare list command parsing
pub fn setup() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .aliases(["l", "ll"])
        .about("List files and directories")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all "Display all files")
            .action(ArgAction::SetTrue))
}

/// Handle list command execution
#[instrument(name = "cli.list", skip_all, fields(options.all = args.get_flag("all")))]
pub fn handle(core: &Core, args: &ArgMatches) -> anyhow::Result<()> {
    // Extract arguments
    let path = args.get_one::<PathBuf>("path")
        .and_then(|path| std::path::absolute(path).ok())
        .unwrap_or(env::current_dir()?);

    // Print files
    let ls_colors = LsColors::from_env().unwrap_or_default();

    let files = core.filesystem().list_content(path)?;

    let list = files
        .filter_ok(|file| args.get_flag("all") ||
            file.path().file_name().is_some_and(|name| !name.to_string_lossy().starts_with('.')))
        .map(|file| Ok(format_file(core, file?, &ls_colors)))
        .collect::<anyhow::Result<List>>()?;

    println!("{list}");

    Ok(())
}

fn format_file(core: &Core, location: Box<dyn Location>, ls_colors: &LsColors) -> Vec<String> {
    let path = location.path();
    let language = core.detect_language(&path);

    let file_name = location.location_name().to_str()
        .unwrap_or_default()
        .to_owned();

    let is_dir = location.location_type()
        .map(|lt| lt.is_dir())
        .unwrap_or_default();

    if io::stdout().is_terminal() {
        // for humans : colored !
        let file_style = location.location_style(ls_colors)
            .map(lscolors::Style::to_crossterm_style)
            .unwrap_or_default();

        let language_style = language.as_ref()
            .map(|language| language.style())
            .unwrap_or_default();

        vec![
            file_style.apply(file_name).to_string(),
            if is_dir {
                let units = core.detect_units_at(&path).iter()
                    .map(|unit| unit.style().apply(unit.kind()).to_string())
                    .join("/");
                
                if units.is_empty() {
                    "unknown".dark_grey().to_string()
                } else {
                    units
                }
            } else {
                language
                    .map(|language| language_style.apply(language).to_string())
                    .unwrap_or_else(|| "unknown".dark_grey().to_string())
            },
            core.qualify_path(&path)
                .map(|content| content.style().apply(if is_dir {
                    format!("{content:#}")
                } else {
                    content.to_string()
                }).to_string())
                .unwrap_or_else(|| "unknown".dark_grey().to_string()),
        ]
    } else {
        // for machines
        vec![
            file_name,
            if is_dir {
                let units = core.detect_units_at(&path).iter()
                    .map(|unit| unit.kind().to_string())
                    .join("/");

                if units.is_empty() {
                    "unknown".to_string()
                } else {
                    units
                }
            } else {
                language
                    .map(|language| language.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            },
            core.qualify_path(&path)
                .map(|content| if is_dir {
                    format!("{content:#}")
                } else {
                    content.to_string()
                })
                .unwrap_or_else(|| "unknown".to_string()),
        ]
    }
}