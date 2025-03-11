use std::env;
use std::path::PathBuf;
use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use tracing::{debug, instrument};
use ring_cli_list::ListFilesIterator;

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
#[instrument(name = "cli.list", skip_all)]
pub fn handle(args: &ArgMatches) -> anyhow::Result<()> {
    // Extract arguments
    let path = args.get_one::<PathBuf>("path")
        .cloned()
        .unwrap_or(env::current_dir()?);

    debug!(message = "detected options",
        options.all = args.get_flag("all")
    );

    // Print files
    let ls_colors = lscolors::LsColors::from_env().unwrap_or_default();
    let mut files = ListFilesIterator::new(path)?;
    
    if args.get_flag("all") {
        files.enable_show_all();
    }
    
    for file in files {
        let file = file?;
        let style = ls_colors.style_for(&file)
            .map(lscolors::Style::to_crossterm_style)
            .unwrap_or_default();
        
        println!("{}", style.apply(file.file_name().unwrap_or_default()));
    }
    
    Ok(())
}