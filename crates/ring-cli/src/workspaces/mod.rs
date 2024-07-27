mod list;
mod current;

use std::env;
use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command, value_parser};
use tracing::{info, warn};
use ring_js_project::JsProject;
use ring_project::Workspace;

pub fn build_command() -> Command {
    Command::new("workspaces")
        .aliases(["workspace", "wks"])
        .subcommands([
            list::build_command(),
            current::build_command()
        ])
        .arg(arg!(-p --project <directory> ... "Project directory. Defaults to current directory")
            .global(true)
            .required(false)
            .value_parser(value_parser!(PathBuf)))
}

#[tracing::instrument(name = "workspaces", skip_all)]
pub fn handle_command(args: &ArgMatches) -> Result<()> {
    // Compute project directory
    let current_dir = env::current_dir()?;
    let project_dir = args.get_one::<PathBuf>("project")
        .unwrap_or(&current_dir);

    let project_dir = current_dir.join(project_dir).canonicalize()
        .context(format!("Unable to access {}", project_dir.display()))?;

    // Search project root
    info!("Searching project root from {}", project_dir.display());

    if let Some(project) = JsProject::search_from(&project_dir)? {
        info!("Project root found at {}", project.root().display());

        match args.subcommand() {
            Some(("list", _)) => list::handle_command(project),
            Some(("current", _)) => current::handle_command(project),
            _ => unreachable!()
        }
    } else {
        warn!("No project root found");
        Ok(())
    }
}