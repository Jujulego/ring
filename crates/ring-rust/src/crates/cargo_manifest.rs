use std::str::FromStr;
use anyhow::Context;
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CargoPackage {
    pub name: String,
    #[serde(default)]
    pub version: Option<Version>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CargoWorkspace {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CargoManifest {
    pub package: Option<CargoPackage>,
    pub workspace: Option<CargoWorkspace>,
}

impl FromStr for CargoManifest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        toml::from_str(s)
            .context("Error while parsing cargo manifest")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_crate_manifest() {
        let manifest = r#"
            [package]
            name = "test"
        "#.parse::<CargoManifest>();

        assert_eq!(manifest.unwrap(), CargoManifest {
            package: Some(CargoPackage {
                name: "test".to_string(),
                version: None
            }),
            workspace: None
        });
    }

    #[test]
    fn it_should_parse_crate_manifest_with_version() {
        let manifest = r#"
            [package]
            name = "test"
            version = "1.0.0"
        "#.parse::<CargoManifest>();

        assert_eq!(manifest.unwrap(), CargoManifest {
            package: Some(CargoPackage {
                name: "test".to_string(),
                version: Some(Version::new(1, 0, 0))
            }),
            workspace: None
        });
    }

    #[test]
    fn it_should_parse_workspace_manifest() {
        let manifest = r#"
            [workspace]
            members = ["crates/test-a", "crates/test-b"]
        "#.parse::<CargoManifest>();

        assert_eq!(manifest.unwrap(), CargoManifest {
            package: None,
            workspace: Some(CargoWorkspace {
                members: vec!["crates/test-a".to_string(), "crates/test-b".to_string()],
            })
        });
    }
}