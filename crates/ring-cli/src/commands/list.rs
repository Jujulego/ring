use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use crossterm::style::{style, Color, Stylize};
use ring_cli_list::FilesIterator;
use ring_core::Core;
use ring_core_language::Language;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::{env, io};
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
    let ls_colors = lscolors::LsColors::from_env().unwrap_or_default();
    let mut files = FilesIterator::new(path)?;
    
    if args.get_flag("all") {
        files.enable_show_all();
    }
    
    for file in files {
        let file = file?;
        let file_name = file.file_name().unwrap_or_default();
        let language = core.detect_language(file.path());
        
        if io::stdout().is_terminal() {
            let file_name = ls_colors.style_for(&file)
                .map(lscolors::Style::to_crossterm_style)
                .unwrap_or_default()
                .apply(file_name);

            let language = language
                .map(|l| {
                    if let Some(color) = l.color() {
                        let color = Color::Rgb { r: color.r, g: color.g, b: color.b };
                        style(l).with(color)
                    } else {
                        style(l)
                    }
                })
                .unwrap_or_else(|| style(unknown_language()).dark_grey());

            println!("{file_name} {language}");
        } else {
            let language = language.map(|l| l.name().to_string())
                .unwrap_or_default();

            println!("{file_name} {language}");
        }
    }
    
    Ok(())
}

pub fn unknown_language() -> Language {
    Language::new("unknown".to_string())
}