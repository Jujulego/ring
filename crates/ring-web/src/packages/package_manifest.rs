use std::io::Read;
use anyhow::Context;
use semver::Version;
use serde::Deserialize;

/// Parsed npm package manifest
#[derive(Clone, Debug, Deserialize)]
pub struct PackageManifest {
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<Version>,
    #[serde(default)]
    pub workspaces: Vec<String>,
}

impl PackageManifest {
    /// Parse given content
    pub fn from_str(content: &str) -> anyhow::Result<Self> {
        serde_json::from_str(content)
            .context("Error while parsing package manifest")
    }

    /// Parse content from given reader
    pub fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        serde_json::from_reader(reader)
            .context("Error while parsing package manifest")
    }
}