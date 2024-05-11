use anyhow::{Context, Result};
use clap::Args;
use std::env;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(long, short)]
    project: Option<PathBuf>,
}

pub fn handle_list_command(args: ListArgs) -> Result<()> {
    println!("{:?}", args.project.unwrap_or(env::current_dir().with_context(|| format!("Current directory unknown"))?));
    
    Ok(())
}