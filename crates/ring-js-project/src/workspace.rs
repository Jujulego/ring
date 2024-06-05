use anyhow::Result;
use std::path::{Path, PathBuf};
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
    
    pub fn get_name(&self) -> &String {
        &self.manifest.name
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }
}