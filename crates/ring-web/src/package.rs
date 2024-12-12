use crate::package_manager::PackageManager;
use crate::package_manifest::PackageManifest;
use crate::WebLanguage;
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::{Tag, Tagged};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

////////////////////////////////////////////////////////////////////////////////
// Package
////////////////////////////////////////////////////////////////////////////////

/// Represents a script package
#[derive(Clone, Debug)]
pub struct Package {
    manifest: PackageManifest,
    package_manager: Option<PackageManager>,
    path: PathBuf,
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
    fn language(&self) -> CodeLanguage {
        WebLanguage::JavaScript.into()
    }

    fn name(&self) -> Option<&str> {
        self.manifest.name.as_deref()
            .or_else(|| self.path.file_name().and_then(OsStr::to_str))
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Tagged for Package {
    fn tags(&self) -> Vec<Tag> {
        let mut tags = vec![];

        if let Some(package_manager) = &self.package_manager {
            tags.push(package_manager.tag());
        }

        tags
    }
}