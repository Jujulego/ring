use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use ring_traits::Manifest;

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

impl Manifest for CargoManifest {
    fn from_str(content: &str) -> anyhow::Result<Self> {
        toml::from_str(content)
            .context("Error while parsing cargo manifest")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_crate_manifest() {
        let manifest = CargoManifest::from_str(r#"
            [package]
            name = "test"
        "#);

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
        let manifest = CargoManifest::from_str(r#"
            [package]
            name = "test"
            version = "1.0.0"
        "#);

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
        let manifest = CargoManifest::from_str(r#"
            [workspace]
            members = ["crates/test-a", "crates/test-b"]
        "#);

        assert_eq!(manifest.unwrap(), CargoManifest {
            package: None,
            workspace: Some(CargoWorkspace {
                members: vec!["crates/test-a".to_string(), "crates/test-b".to_string()],
            })
        });
    }
}