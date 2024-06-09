use std::fmt::{Display, Formatter};
use anyhow::Result;
use std::path::{Path, PathBuf};
use jill_project::Workspace;
use crate::constants::MANIFEST;
use crate::package_manifest::PackageManifest;

#[derive(Debug)]
pub struct JsWorkspace {
    root: PathBuf,
    manifest: PackageManifest,
}

impl JsWorkspace {
    pub fn new(root: &Path) -> Result<JsWorkspace> {
        Ok(JsWorkspace {
            root: root.to_path_buf(),
            manifest: PackageManifest::parse_file(&root.join(MANIFEST))?
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

    fn version(&self) -> Option<&str> {
        self.manifest.version.as_deref()
    }
}

impl Display for JsWorkspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reference())
    }
}