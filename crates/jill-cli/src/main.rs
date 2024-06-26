use std::io;
use anyhow::Result;
use clap::{arg, ArgAction, command};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod js;

fn main() -> Result<()> {
    // Parse args
    let args = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(js::build_command())
        .arg(arg!(-v --verbose)
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
        .with_target(false)
        .with_writer(io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Handle subcommands
    match args.subcommand() {
        Some(("js", args)) => js::handle_command(args),
        _ => unreachable!()
    }
}
