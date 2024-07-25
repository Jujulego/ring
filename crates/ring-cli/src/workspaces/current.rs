use std::env;
use anyhow::{Context, Result};
use clap::Command;
use tracing::{error, warn};
use ring_project::{Project, Workspace};

pub fn build_command() -> Command {
    Command::new("current")
        .visible_alias("pwd")
}

#[tracing::instrument(name = "current", skip_all)]
pub fn handle_command(project: impl Project) -> Result<()> {

    let current_dir = env::current_dir()?;
    let current_dir = current_dir.canonicalize()
        .context(format!("Unable to access {}", current_dir.display()))?;

    let workspace = project.workspaces()
        .flat_map(|wks| wks.map_or_else(|err| { error!("{}", err); None }, Some))
        .filter(|wks| current_dir.starts_with(wks.root()))
        .max_by(|a, b| a.root().components().count().cmp(&b.root().components().count()));

    if let Some(workspace) = workspace {
        println!("{}", workspace.name());
    } else {
        warn!("No matching workspace found");
    }

    Ok(())
}