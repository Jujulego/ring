use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use crossterm::style::{Color, ContentStyle, Stylize};
use ring_cli_fs::FilesIterator;
use ring_cli_list::List;
use ring_core::Core;
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

    let mut list = List::new();
    let mut files = FilesIterator::new(path)?;
    
    if args.get_flag("all") {
        files.enable_show_all();
    }
    
    for file in files {
        let file = file?;
        let file_name = file.file_name().unwrap_or_default().to_string();
        let language = core.detect_language(file.path());
        
        if io::stdout().is_terminal() {
            // for humans : colored !
            let file_style = ls_colors.style_for(&file)
                .map(lscolors::Style::to_crossterm_style)
                .unwrap_or_default();

            let language_style = language.as_ref()
                .map(|language| {
                    let mut style = ContentStyle::new();
                    style.foreground_color = language.color().map(|color| Color::Rgb { r: color.r, g: color.g, b: color.b });

                    style
                })
                .unwrap_or_default();

            list.push(vec![
                file_style.apply(file_name).to_string(),
                language.map(|language| language_style.apply(language).to_string())
                    .unwrap_or_else(|| "<unknown>".dark_grey().to_string())
            ]);
        } else {
            // for machines
            list.push(vec![
                file_name,
                language.map(|language| language.to_string())
                    .unwrap_or_else(|| "<unknown>".to_string()),
            ]);
        }
    }

    println!("{list}");

    Ok(())
}