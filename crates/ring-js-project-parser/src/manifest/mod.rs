use std::collections::HashMap;
use std::io;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub workspaces: Vec<String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

pub enum Error {
    Io(io::Error),
    Parse(serde_json::Error)
}

impl PackageManifest {
    pub fn parse_file(path: &str) -> Result<PackageManifest, Error> {
        let file = File::open(path).map_err(Error::Io)?;
        let manifest = serde_json::from_reader(&file).map_err(Error::Parse)?;
        
        Ok(manifest)
    }
}