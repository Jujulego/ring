use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use semver::Version;

use ring_project::{Dependency, Workspace};

use crate::constants::MANIFEST;
use crate::package_manifest::PackageManifest;

#[derive(Debug)]
pub struct JsWorkspace {
    root: PathBuf,
    manifest: PackageManifest,
    dependencies: Vec<Dependency>,
}

impl JsWorkspace {
    pub fn new(root: &Path) -> Result<JsWorkspace> {
        let manifest = PackageManifest::parse_file(&root.join(MANIFEST))?;
        
        Ok(JsWorkspace {
            root: root.to_path_buf(),
            dependencies: manifest.dependencies(root)?,
            manifest,
        })
    }

    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
}

impl Workspace for JsWorkspace {
    fn name(&self) -> &str {
        &self.manifest.name
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn version(&self) -> Option<&Version> {
        self.manifest.version.as_ref()
    }

    fn dependencies(&self) -> &Vec<Dependency> {
        &self.dependencies
    }
}

impl Display for JsWorkspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reference())
    }
}