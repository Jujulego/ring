use std::path::{Path, PathBuf};
use semver::Version;
use ring_traits::Project;
use crate::CargoPackage;

#[derive(Debug)]
pub struct RustProject {
    root: PathBuf,
    package: CargoPackage,
}

impl RustProject {
    pub fn new(root: PathBuf, package: CargoPackage) -> RustProject {
        RustProject { root, package }
    }

    pub fn package(&self) -> &CargoPackage {
        &self.package
    }
}

impl Project for RustProject {
    fn root(&self) -> &Path {
        &self.root
    }

    fn name(&self) -> &str {
        &self.package.name
    }

    fn version(&self) -> Option<&Version> {
        self.package.version.as_ref()
    }

    fn tags(&self) -> &[&str] {
        &["rust"]
    }
}