mod dependency;

use std::collections::HashMap;
use semver::Version;
use serde::Deserialize;

pub use crate::workspace::dependency::{WorkspaceDependency, WorkspaceKind, WorkspaceRequirement};

#[derive(Debug, Deserialize)]
struct WorkspaceManifest {
    name: String,
    version: String,
    #[serde(default)] dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")] dev_dependencies: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Workspace {
    name: String,
    version: Version,
    dependencies: Vec<WorkspaceDependency>,
}

#[derive(Debug)]
pub enum ParseManifestError {
    JsonError(serde_json::Error),
    VersionError(semver::Error),
    WorkspaceReqError(semver::Error)
}

impl Workspace {
    pub fn parse_manifest(json: &str) -> Result<Workspace, ParseManifestError> {
        let manifest: WorkspaceManifest = serde_json::from_str(json)
            .map_err(ParseManifestError::JsonError)?;
        
        let dependencies = Self::parse_dependencies(&manifest)?;

        Ok(Workspace {
            name: manifest.name,
            version: Version::parse(&manifest.version).map_err(ParseManifestError::VersionError)?,
            dependencies,
        })
    }

    fn parse_dependencies(manifest: &WorkspaceManifest) -> Result<Vec<WorkspaceDependency>, ParseManifestError> {
        let mut result = Vec::new();

        for (name, requirement) in &manifest.dependencies {
            result.push(
                WorkspaceDependency::new(name, WorkspaceKind::Prod, requirement)
                    .map_err(ParseManifestError::WorkspaceReqError)?
            );
        }

        for (name, requirement) in &manifest.dev_dependencies {
            result.push(
                WorkspaceDependency::new(name, WorkspaceKind::Dev, requirement)
                    .map_err(ParseManifestError::WorkspaceReqError)?
            );
        }
        
        Ok(result)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }

    pub fn get_dependencies(&self) -> &Vec<WorkspaceDependency> {
        &self.dependencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_manifest() {
        let json = r#"{
            "name": "test",
            "version": "1.0.0",
            "description": "Lorem ipsum dolor sit amet"
        }"#;

        let workspace = Workspace::parse_manifest(json).unwrap();

        assert_eq!(workspace.get_name(), "test");
        assert_eq!(workspace.get_version(), &"1.0.0".parse().unwrap());
    }

    #[test]
    fn it_should_parse_manifest_with_dependencies() {
        let json = r#"{
            "name": "test",
            "version": "1.0.0",
            "description": "Lorem ipsum dolor sit amet",
            "dependencies": {
              "life": "42.0.0"
            }
        }"#;

        let workspace = Workspace::parse_manifest(json).unwrap();

        assert_eq!(workspace.get_name(), "test");
        assert_eq!(workspace.get_version(), &"1.0.0".parse().unwrap());
        
        assert_eq!(workspace.get_dependencies()[0].get_name(), "life");
        assert_eq!(workspace.get_dependencies()[0].get_kind(), &WorkspaceKind::Prod);
        assert_eq!(workspace.get_dependencies()[0].get_requirement(), &WorkspaceRequirement::Npm("42.0.0".parse().unwrap()));
    }

    #[test]
    fn it_should_parse_manifest_with_dev_dependencies() {
        let json = r#"{
            "name": "test",
            "version": "1.0.0",
            "description": "Lorem ipsum dolor sit amet",
            "devDependencies": {
              "life": "42.0.0"
            }
        }"#;

        let workspace = Workspace::parse_manifest(json).unwrap();

        assert_eq!(workspace.get_name(), "test");
        assert_eq!(workspace.get_version(), &"1.0.0".parse().unwrap());

        assert_eq!(workspace.get_dependencies()[0].get_name(), "life");
        assert_eq!(workspace.get_dependencies()[0].get_kind(), &WorkspaceKind::Dev);
        assert_eq!(workspace.get_dependencies()[0].get_requirement(), &WorkspaceRequirement::Npm("42.0.0".parse().unwrap()));
    }
}
