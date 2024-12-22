use crate::packages::package_manager::PackageManager;
use crate::packages::package_manifest::PackageManifest;
use crate::WebLanguage;
use ring_core::{CodeLanguage, CodeUnit};
use ring_tag::{Tag, Tagged};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Represents a web package
#[derive(Clone, Debug)]
pub struct Package {
    manifest: PackageManifest,
    package_manager: Option<PackageManager>,
    path: PathBuf,
}

impl Package {
    /// Builds a new web package
    pub fn new(path: PathBuf, manifest: PackageManifest) -> Package {
        Package { path, manifest, package_manager: None }
    }

    /// Returns parsed manifest of this package
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }

    /// Returns name of package, from its manifest. If not defined in the manifest,
    /// it gives the directory name.
    pub fn name(&self) -> Option<&str> {
        self.manifest.name.as_deref()
            .or_else(|| self.path.file_name().and_then(OsStr::to_str))
    }

    /// Returns detected package manager (based on lockfile)
    /// Returns [`PackageManager::Npm`] if none is detected
    pub fn package_manager(&self) -> Option<&PackageManager> {
        self.package_manager.as_ref()
    }

    pub fn package_manager_mut(&mut self) -> &mut Option<PackageManager> {
        &mut self.package_manager
    }

    /// Returns path to this script
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl CodeUnit for Package {
    fn language(&self) -> CodeLanguage {
        WebLanguage::JavaScript.into()
    }

    fn name(&self) -> Option<&str> {
        self.name()
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