use std::path::{Path, PathBuf};
use std::rc::Rc;
use semver::Version;
use ring_traits::{Project, Tagged};
use crate::package_manifest::PackageManifest;

#[derive(Debug)]
pub struct JsProject {
    root: PathBuf,
    manifest: Rc<PackageManifest>,
}

impl JsProject {
    pub fn new(root: PathBuf, manifest: Rc<PackageManifest>) -> JsProject {
        JsProject { root, manifest }
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