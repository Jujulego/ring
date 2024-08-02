use std::cmp::max;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use anyhow::Context;
use clap::{arg, ArgAction, ArgMatches, Command, value_parser};
use colored::Colorize;
use tracing::info;
use ring_js::JsProjectDetector;
use ring_traits::{Project, ProjectDetector};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!(-a --all)
            .action(ArgAction::SetTrue))
}

pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let path = current_dir.join(path).canonicalize()
        .with_context(|| format!("Unable to access {}", path.display()))?;

    let show_all = args.get_one::<bool>("all").unwrap_or(&false);

    // List directory files
    let detector = JsProjectDetector::new();

    if path.is_dir() {
        info!("Searching project root from {}", path.display());
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

            if let Some(project) = detector.detect_from(&entry.path())? {
                first_len = max(first_len, project.name().len());

                results.push((file_name, project.name().normal()));
            } else {
                first_len = max(first_len, 7);
                
                results.push((file_name, "unknown".bright_black()));
            }
        }
        
        for (file_name, workspace) in results {
            println!("{:first_len$} {}", workspace, file_name);
        }
    } else {
        let project = detector.detect_from(&path)?;

        println!(
            "{} {}",
            path.file_name().and_then(|s| s.to_str()).unwrap(),
            project.map_or("unknown".bright_black(), |wks| wks.name().normal())
        );
    }

    Ok(())
}
