use anyhow::{Context, Result};
use clap::Args;
use std::env;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[command(alias = "ls")]
/// List workspaces of a js project
pub struct ListArgs {
    #[arg(long, short)]
    /// Project directory. Defaults to current directory
    project: Option<PathBuf>,
}

pub fn handle_list_command(args: ListArgs) -> Result<()> {
    println!("{:?}", args.project.unwrap_or(env::current_dir().with_context(|| format!("Current directory unknown"))?));

    Ok(())
}