use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WorkspaceManifest {
    name: String,
    version: String,
}

#[derive(Debug)]
pub struct Workspace {
    name: String,
    version: Version,
}

#[derive(Debug)]
pub enum ParseManifestError {
    JsonError(serde_json::Error),
    VersionError(semver::Error)
}

impl Workspace {
    pub fn parse_manifest(json: &str) -> Result<Workspace, ParseManifestError> {
        let manifest: WorkspaceManifest = serde_json::from_str(json)
            .map_err(ParseManifestError::JsonError)?;

        Ok(Workspace {
            name: manifest.name,
            version: Version::parse(&manifest.version)
                .map_err(ParseManifestError::VersionError)?,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &Version {
        &self.version
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
}