use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use crossterm::style::Stylize;
use lscolors::LsColors;
use ring_cli_fs::{FilesItem, FilesIterator};
use ring_cli_list::List;
use ring_core::Core;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::{env, io};
use itertools::Itertools;
use tracing::instrument;

/// Prepare list command parsing
pub fn setup() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .aliases(["l", "ll"])
        .about("List files and directories (in current directory by default)")
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
        .cloned()
        .unwrap_or(env::current_dir()?);

    // Print files
    let ls_colors = LsColors::from_env().unwrap_or_default();

    let mut files = FilesIterator::new(path)?;
    
    if args.get_flag("all") {
        files.enable_show_all();
    }

    let list = files
        .map(|file| Ok(format_file(core, file?, &ls_colors)))
        .collect::<anyhow::Result<List>>()?;

    println!("{list}");

    Ok(())
}

fn format_file(core: &Core, file: FilesItem, ls_colors: &LsColors) -> Vec<String> {
    let file_name = file.file_name().unwrap_or_default().to_string();
    let language = core.detect_language(file.path());

    let is_dir = file.metadata().map(|mtd| mtd.is_dir()).unwrap_or_default();

    if io::stdout().is_terminal() {
        // for humans : colored !
        let file_style = ls_colors.style_for(&file)
            .map(lscolors::Style::to_crossterm_style)
            .unwrap_or_default();

        let language_style = language.as_ref()
            .map(|language| language.style())
            .unwrap_or_default();

        vec![
            file_style.apply(file_name).to_string(),
            language
                .map(|language| language_style.apply(language).to_string())
                .unwrap_or_else(|| if is_dir { "directory".dim() } else { "unknown".dark_grey() }.to_string()),
            if is_dir {
                let units = core.detect_units(file.path()).iter()
                    .map(|unit| unit.style().apply(unit.kind()).to_string())
                    .join(",");
                
                if units.is_empty() {
                    "unknown".dark_grey().to_string()
                } else {
                    units
                }
            } else {
                core.qualify_content(file.path())
                    .map(|content| content.style().apply(content).to_string())
                    .unwrap_or_else(|| "unknown".dark_grey().to_string())
            },
        ]
    } else {
        // for machines
        vec![
            file_name,
            language
                .map(|language| language.to_string())
                .unwrap_or_else(|| if is_dir { "directory" } else { "unknown" }.to_string()),

            if is_dir {
                let units = core.detect_units(file.path()).iter()
                    .map(|unit| unit.kind().to_string())
                    .join(",");

                if units.is_empty() {
                    "unknown".to_string()
                } else {
                    units
                }
            } else {
                core.qualify_content(file.path())
                    .map(|content| content.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            },
        ]
    }
}