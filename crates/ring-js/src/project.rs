use std::fs::File;
use std::path::{Path, PathBuf};
use anyhow::Context;
use semver::Version;
use tracing::trace;
use ring_traits::{Manifest, Project, Tagged};
use crate::constants::MANIFEST;
use crate::package_manifest::PackageManifest;

#[derive(Debug)]
pub struct JsProject {
    root: PathBuf,
    manifest: PackageManifest,
}

impl JsProject {
    pub fn new(root: PathBuf) -> anyhow::Result<JsProject> {
        let manifest_path = root.join(MANIFEST);

        trace!("Parsing manifest file {}", manifest_path.display());
        let mut manifest = File::open(&manifest_path)
            .with_context(|| format!("Unable to read file {}", manifest_path.display()))?;

        Ok(JsProject {
            root,
            manifest: PackageManifest::from_reader(&mut manifest)?,
        })
    }
    
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
}

impl Project for JsProject {
    fn root(&self) -> &Path {
        &self.root
    }

    fn name(&self) -> &str {
        &self.manifest.name
    }

    fn version(&self) -> Option<&Version> {
        self.manifest.version.as_ref()
    }
}

impl Tagged for JsProject {
    fn tags(&self) -> &[&'static str] {
        &["js"]
    }
}