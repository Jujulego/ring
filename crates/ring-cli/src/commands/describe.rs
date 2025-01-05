use crate::core::RingCore;
use clap::{arg, value_parser, ArgMatches, Command};
use crossterm::style::Stylize;
use std::env;
use std::path::PathBuf;
use itertools::Itertools;
use tracing::instrument;
use ring_tag::Tag;

pub fn build_command() -> Command {
    Command::new("describe")
        .aliases(["d", "desc"])
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
}

#[instrument(name = "cli.describe", skip(core, args))]
pub fn handle_command(core: &RingCore, args: &ArgMatches) -> anyhow::Result<()> {
    // Extract arguments
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    for detector in core.code_unit_detectors() {
        if let Some(unit) = detector.detect(path)? {
            println!("{} detected by {}",
                     unit.name().map(Stylize::bold).unwrap_or("unknown".dark_grey()),
                     detector.name());

            if let Some(parent) = unit.parent() {
                println!("\u{2570}\u{2574}{} at {}",
                         parent.name().map(Stylize::bold).unwrap_or("unknown".dark_grey()),
                         parent.path().display());
            }

            println!("language: {}", Tag::from(unit.language()).stylize());
            println!("tags:     {}", unit.tags().iter().map(Tag::stylize_full).join(" "));
            println!();
        }
    }

    Ok(())
}