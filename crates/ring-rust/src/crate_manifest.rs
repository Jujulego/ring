use std::fs::File;
use std::io::Read;
use std::path::Path;
use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct CratePackage {
    pub name: String,
    #[serde(default)]
    pub version: Option<Version>,
}

#[derive(Debug, Deserialize)]
pub struct CreateManifest {
    pub package: CratePackage
}

impl CreateManifest {
    pub fn parse_file(path: &Path) -> anyhow::Result<CreateManifest> {
        trace!("Parsing manifest file {}", path.display());

        let mut buffer = String::new();
        
        File::open(path)
            .and_then(|mut f| f.read_to_string(&mut buffer))
            .with_context(|| format!("Unable to read file {}", path.display()))?;

        toml::from_str(&buffer)
            .with_context(|| format!("Error while parsing {}", path.display()))
    }
}
