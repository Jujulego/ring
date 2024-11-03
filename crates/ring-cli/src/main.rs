use clap::{arg, ArgAction, ArgMatches, Command};
use std::io;
use tracing::Level;
use tracing_subscriber::prelude::*;

fn main() -> anyhow::Result<()> {
    let _guard = setup_sentry();

    // Parse arguments
    let args = Command::new("ring")
        .version(env!("RING_CLI_VERSION"))
        .propagate_version(true)
        .arg(arg!(-v --verbose)
            .global(true)
            .required(false)
            .action(ArgAction::Count))
        .get_matches();

    // Setup tracing
    setup_tracing(&args);

    Ok(())
}

fn setup_sentry() -> sentry::ClientInitGuard {
    sentry::init(("https://c9f17af2112f77c5e940111c1aade8db@o4508229080055808.ingest.de.sentry.io/4508236598214736", sentry::ClientOptions {
        release: Some(format!("ring@{}", env!("RING_CLI_VERSION")).into()),
        traces_sample_rate: 1.0,
        ..Default::default()
    }))
}

fn setup_tracing(args: &ArgMatches) {
    tracing_subscriber::registry()
        .with(sentry::integrations::tracing::layer())
        .with(tracing_subscriber::fmt::layer()
            .without_time()
            .with_target(false)
            .with_writer(io::stderr
                .with_max_level(match dbg!(args.get_count("verbose")) {
                    0 => Level::WARN,
                    1 => Level::INFO,
                    2 => Level::DEBUG,
                    _ => Level::TRACE,
                })
            )
        )
        .init();
}