use crate::WebLanguage;
use ring_code_unit::CodeUnit;
use ring_tag::{Tag, Tagged};
use std::path::{Path, PathBuf};
use crate::package_manager::PackageManager;

////////////////////////////////////////////////////////////////////////////////
// Package
////////////////////////////////////////////////////////////////////////////////

/// Represents a script package
#[derive(Clone, Debug)]
pub struct Package {
    path: PathBuf,
    package_manager: Option<PackageManager>,
}

impl Package {
    pub fn new(path: PathBuf) -> Package {
        Package { path, package_manager: None }
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