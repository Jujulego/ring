use std::cmp::max;
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use anyhow::Context;
use clap::{arg, ArgAction, ArgMatches, Command, value_parser};
use colored::Colorize;
use tracing::{error, info};
use ring_js_project::JsProject;
use ring_project::{Project, Workspace};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

#[tracing::instrument(name = "list", skip_all)]
pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let directory = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let path = current_dir.join(directory).canonicalize()
        .context(format!("Unable to access {}", directory.display()))?;
    
    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List directory files
    if path.is_dir() {
        info!("Searching project root from {}", path.display());
        let project = JsProject::search_from(&path)?;
        let mut first_len = 0;
        let mut results = Vec::new();

        for entry in read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_str().unwrap().to_string();
            
            if !show_all && file_name.starts_with('.') {
                continue;
            }
            
            let file_name = {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        file_name.blue()
                    } else if file_type.is_symlink() {
                        file_name.cyan()
                    } else {
                        file_name.normal()
                    }
                } else {
                    file_name.normal()
                }
            };
            
            let workspace = if let Some(project) = &project { find_workspace(&entry.path(), project) } else { None };
            
            if let Some(workspace) = workspace {
                first_len = max(first_len, workspace.name().len());

                results.push((file_name, workspace.name().normal()));
            } else {
                first_len = max(first_len, 7);
                
                results.push((file_name, "unknown".bright_black()));
            }
        }
        
        for (file_name, workspace) in results {
            println!("{:first_len$} {}", workspace, file_name);
        }
    } else {
        info!("Searching project root from {}", path.display());
        let workspace = JsProject::search_from(path.parent().unwrap())?
            .and_then(|prj| find_workspace(&path, &prj));

        println!(
            "{} {}",
            path.file_name().and_then(|s| s.to_str()).unwrap(),
            workspace.map_or("unknown".bright_black(), |wks| wks.name().normal())
        );
    }

    Ok(())
}

fn find_workspace<P: Project>(path: &Path, project: &P) -> Option<Rc<P::Workspace>> {
    project.workspaces()
        .flat_map(|wks| wks.map_or_else(|err| { error!("{}", err); None }, Some))
        .filter(|wks| path.starts_with(wks.root()))
        .max_by(|a, b| a.root().components().count().cmp(&b.root().components().count()))
}