use std::io::Read;
use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use ring_traits::Manifest;

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    #[serde(default)]
    pub version: Option<Version>,
    #[serde(default)]
    pub workspaces: Vec<String>,
}

impl Manifest for PackageManifest {
    fn from_str(content: &str) -> anyhow::Result<Self> {
        serde_json::from_str(content)
            .context("Error while parsing package manifest")
    }

    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        serde_json::from_reader(reader)
            .context("Error while parsing package manifest")
    }
}