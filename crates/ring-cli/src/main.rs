use std::io;
use clap::{arg, ArgAction, Command};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()> {
    // Setup commands
    let args = Command::new("ring")
        .version(env!("RING_CLI_VERSION"))
        .propagate_version(true)
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
    
    Ok(())
}
