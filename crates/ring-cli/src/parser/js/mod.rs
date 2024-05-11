use anyhow::Result;
use clap::{Args, Subcommand};
use crate::parser::js::list::{handle_list_command, ListArgs};

mod list;

#[derive(Debug, Subcommand)]
pub enum JsCommands {
    List(ListArgs)
}

#[derive(Args, Debug)]
pub struct JsCli {
    #[command(subcommand)]
    command: JsCommands,
}

pub fn handle_js_command(cli: JsCli) -> Result<()> {
    match cli.command {
        JsCommands::List(args) => handle_list_command(args),
    }
}