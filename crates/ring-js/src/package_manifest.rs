use anyhow::Context;
use ring_traits::Manifest;
use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PackageManifest {
    pub name: String,
    #[serde(default)]
    pub version: Option<Version>,
    #[serde(default)]
    pub workspaces: Vec<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default, rename="devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(default, rename="optionalDependencies")]
    pub optional_dependencies: HashMap<String, String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_package_manifest() {
        let manifest = PackageManifest::from_str(r#"{
            "name": "test"
        }"#);

        assert_eq!(manifest.unwrap(), PackageManifest {
            name: "test".to_string(),
            version: None,
            workspaces: Vec::new(),
            dependencies: HashMap::default(),
            dev_dependencies: HashMap::default(),
            optional_dependencies: HashMap::default(),
        });
    }

    #[test]
    fn it_should_parse_package_manifest_with_version() {
        let manifest = PackageManifest::from_str(r#"{
            "name": "test",
            "version": "1.0.0"
        }"#);

        assert_eq!(manifest.unwrap(), PackageManifest {
            name: "test".to_string(),
            version: Some(Version::new(1, 0, 0)),
            workspaces: Vec::new(),
            dependencies: HashMap::default(),
            dev_dependencies: HashMap::default(),
            optional_dependencies: HashMap::default(),
        });
    }

    #[test]
    fn it_should_parse_package_manifest_with_workspace() {
        let manifest = PackageManifest::from_str(r#"{
            "name": "test",
            "workspaces": ["packages/test-a", "packages/test-b"]
        }"#);

        assert_eq!(manifest.unwrap(), PackageManifest {
            name: "test".to_string(),
            version: None,
            workspaces: vec![
                "packages/test-a".to_string(),
                "packages/test-b".to_string()
            ],
            dependencies: HashMap::default(),
            dev_dependencies: HashMap::default(),
            optional_dependencies: HashMap::default(),
        });
    }
}