use anyhow::Result;
use clap::Parser;
use crate::parser::{Cli, handle_cli};

mod parser;

fn main() -> Result<()> {
    handle_cli(Cli::parse())
}
