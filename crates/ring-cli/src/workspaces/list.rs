use std::cmp::max;
use anyhow::Result;
use clap::Command;
use colored::Colorize;
use ring_project::{Project, Workspace};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
}

#[tracing::instrument(name = "list", skip_all)]
pub fn handle_command(project: impl Project) -> Result<()> {
    let mut workspaces = Vec::new();
    let mut name_len = 0;

    for workspace in project.workspaces() {
        let workspace = workspace?;

        name_len = max(name_len, workspace.name().len());
        workspaces.push(workspace);
    }

    for workspace in workspaces {
        let version = workspace.version()
            .map_or("unknown".bright_black(), |v| v.to_string().normal());

        println!("{:name_len$} {}", workspace.name(), version, name_len = name_len);
    }

    Ok(())
}