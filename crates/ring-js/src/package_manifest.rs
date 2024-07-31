use std::fs::File;
use std::path::Path;
use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    #[serde(default, with = "serde_version")]
    pub version: Option<Version>,
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

// Parse version number
mod serde_version {
    use semver::Version;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Version>, D::Error>
    where
        D: Deserializer<'de>
    {
        let s: Option<String> = Option::deserialize(deserializer)?;

        if let Some(s) = s {
            return Ok(Some(Version::parse(&s).map_err(serde::de::Error::custom)?))
        }
        
        Ok(None)
    }
}
