use anyhow::Context;
use clap::{arg, value_parser, ArgMatches, Command};
use itertools::Itertools;
use ring_code_unit::CodeUnitDetector;
use ring_web::WebUnitDetector;
use std::collections::BTreeSet;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use tracing::{instrument, span, Level};

pub fn build_command() -> Command {
    Command::new("list")
        .visible_alias("ls")
        .arg(arg!([path])
            .value_parser(value_parser!(PathBuf)))
}

#[instrument(name = "list")]
pub fn handle_command(args: &ArgMatches) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = args.get_one::<PathBuf>("path")
        .unwrap_or(&current_dir);

    let detectors: &[Box<dyn CodeUnitDetector>] = &[
        Box::new(WebUnitDetector {})
    ];

    for path in list_files(path)? {
        let mut tags = BTreeSet::new();

        for detector in detectors {
            if let Some(unit) = detector.detect(&path)? {
                tags.extend(unit.tags());
            }
        }

        println!("{} {}", path.file_name().unwrap().to_str().unwrap(), tags.iter().map(|tag| tag.styled()).join(" "));
    }

    Ok(())
}

fn list_files(path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    if path.is_dir() {
        span!(Level::TRACE, "read_dir").in_scope(|| {
            read_dir(path)?
                .map(|res| res
                    .map(|e| e.path())
                    .with_context(|| format!("Error while reading directory {:?}", path))
                )
                .collect()
        })
    } else {
        Ok(vec![path.clone()])
    }
}