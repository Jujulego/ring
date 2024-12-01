use crate::WebLanguage;
use ring_code_unit::CodeUnit;
use ring_tag::{Tag, Tagged};
use std::path::{Path, PathBuf};
use crate::package_manager::PackageManager;
use crate::package_manifest::PackageManifest;
////////////////////////////////////////////////////////////////////////////////
// Package
////////////////////////////////////////////////////////////////////////////////

/// Represents a script package
#[derive(Clone, Debug)]
pub struct Package {
    path: PathBuf,
    manifest: PackageManifest,
    package_manager: Option<PackageManager>,
}

impl Package {
    pub fn new(path: PathBuf, manifest: PackageManifest) -> Package {
        Package { path, manifest, package_manager: None }
    }

    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
    
    pub fn package_manager(&self) -> Option<&PackageManager> {
        self.package_manager.as_ref()
    }

    pub fn set_package_manager(&mut self, package_manager: PackageManager) {
        self.package_manager = Some(package_manager);
    }
}

impl CodeUnit for Package {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for Package {
    fn tags(&self) -> Vec<Tag> {
        let mut tags = vec![WebLanguage::JavaScript.tag()];

        if let Some(package_manager) = &self.package_manager {
            tags.push(package_manager.tag());
        }

        tags
    }
}