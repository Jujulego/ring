use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command, value_parser};
use std::env;
use std::path::PathBuf;
use tracing::{info, warn};
use jill_js_project::JsProject;
use jill_project::Workspace;

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
    
    let project_dir = current_dir.join(project_dir).canonicalize()
        .context(format!("Unable to access {}", project_dir.display()))?;
    
    // Search project root
    info!("Searching project root from {}", project_dir.display());

    if let Some(project) = JsProject::search_from(&project_dir)? {
        info!("Project root found at {}", project.root().display());
        println!("Project {project}");
        
        for workspace in project.list_workspaces()? {
            println!("- {workspace}");
        }
    } else {
        warn!("Project root not found");
    }
    
    Ok(())
}