use std::path::{Path, PathBuf};
use std::rc::Rc;
use semver::Version;
use ring_traits::{Project, Tagged};
use crate::{CargoManifest, CargoPackage};

#[derive(Debug)]
pub struct RustProject {
    root: PathBuf,
    manifest: Rc<CargoManifest>,
}

impl RustProject {
    pub fn new(root: PathBuf, manifest: Rc<CargoManifest>) -> RustProject {
        RustProject { root, manifest }
    }

    pub fn package(&self) -> &CargoPackage {
        self.manifest.package.as_ref().unwrap()
    }
}

impl Project for RustProject {
    fn root(&self) -> &Path {
        &self.root
    }

    fn name(&self) -> &str {
        &self.package().name
    }

    fn version(&self) -> Option<&Version> {
        self.package().version.as_ref()
    }
}

impl Tagged for RustProject {
    fn tags(&self) -> &[&'static str] {
        &["rust"]
    }
}