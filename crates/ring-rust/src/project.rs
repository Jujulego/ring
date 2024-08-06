use std::path::{Path, PathBuf};
use semver::Version;
use ring_traits::Project;
use crate::cargo_manifest::CargoManifest;
use crate::constants::MANIFEST;

#[derive(Debug)]
pub struct RustProject {
    root: PathBuf,
    manifest: CargoManifest,
}

impl RustProject {
    pub fn new(root: PathBuf) -> anyhow::Result<RustProject> {
        let manifest = CargoManifest::parse_file(&root.join(MANIFEST))?;
        
        Ok(RustProject {
            root,
            manifest
        })
    }
    
    pub fn manifest(&self) -> &CargoManifest {
        &self.manifest
    }
}

impl Project for RustProject {
    fn root(&self) -> &Path {
        &self.root
    }

    fn name(&self) -> &str {
        &self.manifest.package.name
    }

    fn version(&self) -> Option<&Version> {
        self.manifest.package.version.as_ref()
    }

    fn tags(&self) -> &[&str] {
        &["rust"]
    }
}