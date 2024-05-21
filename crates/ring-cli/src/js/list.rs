use anyhow::Result;
use clap::{arg, ArgMatches, Command, value_parser};
use std::env;
use std::path::PathBuf;
use tracing::info;

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!(-p --project <directory> ... "Project directory. Defaults to current directory")
            .required(false)
            .value_parser(value_parser!(PathBuf)))
}

#[tracing::instrument(name = "list", skip_all)]
pub fn handle_command(args: &ArgMatches) -> Result<()> {
    let current_dir = env::current_dir()?;

    // Compute project directory
    let project_dir = args.get_one::<PathBuf>("project")
        .unwrap_or(&current_dir);
    
    let project_dir = if project_dir.is_absolute() { 
        project_dir.clone()
    } else {
        current_dir.join(project_dir)
    };
    
    let project_dir = project_dir.canonicalize()?;
    info!("project directory: {}", project_dir.display());
    
    Ok(())
}