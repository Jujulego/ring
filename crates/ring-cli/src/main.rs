use std::io;
use anyhow::Result;
use clap::{arg, ArgAction, Command};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ring_core::RingCore;

mod list;
mod modules;
mod projects;

fn main() -> Result<()> {
    // Setup commands
    let args = Command::new("ring")
        .version(env!("RING_CLI_VERSION"))
        .propagate_version(true)
        .subcommand_required(true)
        .subcommands([
            list::build_command(),
            modules::build_command(),
            projects::build_command()
        ])
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
    let core = RingCore::new();
    
    match args.subcommand() {
        Some(("list", args)) => list::handle_command(&core, args),
        Some(("modules", args)) => modules::handle_command(&core, args),
        Some(("projects", args)) => projects::handle_command(&core, args),
        _ => unreachable!()
    }
}
