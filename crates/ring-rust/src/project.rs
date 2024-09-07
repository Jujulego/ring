use crate::constants::rust_tag;
use crate::{CargoManifest, CargoPackage};
use ring_traits::Project;
use ring_utils::{NormalizedPath, NormalizedPathBuf, Tag, Tagged};
use semver::Version;
use std::rc::Rc;

#[derive(Debug)]
pub struct RustProject {
    root: NormalizedPathBuf,
    manifest: Rc<CargoManifest>,
}

impl RustProject {
    pub fn new(root: NormalizedPathBuf, manifest: Rc<CargoManifest>) -> RustProject {
        RustProject { root, manifest }
    }

    pub fn package(&self) -> &CargoPackage {
        self.manifest.package.as_ref().unwrap()
    }
}

impl Project for RustProject {
    fn root(&self) -> &NormalizedPath {
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
    fn tags(&self) -> Vec<Tag> {
        vec![rust_tag()]
    }
}