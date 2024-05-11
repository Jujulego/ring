use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::parser::js::{handle_js_command, JsCli};

mod js;

#[derive(Debug, Subcommand)]
pub enum CliCommands {
    Js(JsCli)
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

pub fn handle_cli(cli: Cli) -> Result<()> {
    match cli.command {
        CliCommands::Js(args) => handle_js_command(args)
    }
}