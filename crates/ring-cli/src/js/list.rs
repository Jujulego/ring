use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command, value_parser};
use std::env;
use std::path::{Path, PathBuf};
use tracing::{debug, info, trace, warn};

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
    let mut root = if project_dir.is_file() { project_dir.parent().unwrap() } else { &project_dir };
    let mut manifest = None;
    
    loop {
        trace!("Testing {}", root.display());
        
        for lockfile in ["package-lock.json", "yarn.lock"] {
            let lockfile = root.to_path_buf().join(lockfile);
            
            if lockfile.try_exists().context(format!("Unable to access {}", lockfile.display()))? {
                debug!("Found lockfile {}", lockfile.display());
                info!("Project root is {}", root.display());
                return Ok(());
            }
        }

        {
            let file = root.to_path_buf().join("package.json");
            
            if file.try_exists().context(format!("Unable to access {}", file.display()))? {
                debug!("Found manifest {}", file.display());
                manifest = Some(file);
            }
        }
        
        // Move up
        if let Some(parent) = root.parent() {
            root = parent;
        } else {
            break;
        }
    }
    
    if let Some(root) = manifest.as_deref().and_then(Path::parent) {
        info!("Project root is {}", root.display());
        return Ok(());
    }
    
    warn!("Project root not found");
    Ok(())
}