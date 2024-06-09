use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub workspaces: Vec<String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

impl PackageManifest {
    pub fn parse_file(path: &Path) -> Result<PackageManifest> {
        trace!("Parsing manifest file {}", path.display());

        let file = File::open(path).context(format!("Unable to read file {}", path.display()))?;
        let manifest = serde_json::from_reader(&file).context(format!("Error while parsing {}", path.display()))?;

        Ok(manifest)
    }
}