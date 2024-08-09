use std::fs::File;
use std::path::Path;
use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    #[serde(default)]
    pub version: Option<Version>,
    #[serde(default)]
    pub workspaces: Vec<String>,
}

impl PackageManifest {
    pub fn parse_file(path: &Path) -> anyhow::Result<PackageManifest> {
        trace!("Parsing manifest file {}", path.display());

        let file = File::open(path)
            .with_context(|| format!("Unable to read file {}", path.display()))?;

        serde_json::from_reader(&file)
            .with_context(|| format!("Error while parsing {}", path.display()))
    }
}
