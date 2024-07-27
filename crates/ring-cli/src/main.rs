use std::io;
use anyhow::Result;
use clap::{arg, ArgAction, command};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod workspaces;

fn main() -> Result<()> {
    // Parse args
    let args = command!("ring")
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(workspaces::build_command())
        .arg(arg!(-v --verbose)
            .global(true)
            .required(false)
            .action(ArgAction::Count))
        .get_matches();

    // Setup tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(match args.get_count("verbose") {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .without_time()
        .with_target(false)
        .with_writer(io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Handle subcommands
    match args.subcommand() {
        Some(("workspaces", args)) => workspaces::handle_command(args),
        _ => unreachable!()
    }
}
