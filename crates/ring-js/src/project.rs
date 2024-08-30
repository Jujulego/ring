use crate::constants::JS_TAG;
use crate::package_manifest::PackageManifest;
use ring_traits::{Project, Tagged};
use ring_utils::{NormalizedPath, NormalizedPathBuf, Tag};
use semver::Version;
use std::rc::Rc;
use crate::PackageManager;

#[derive(Debug)]
pub struct JsProject {
    root: NormalizedPathBuf,
    manifest: Rc<PackageManifest>,
    package_manager: PackageManager,
}

impl JsProject {
    pub fn new(root: NormalizedPathBuf, manifest: Rc<PackageManifest>, package_manager: PackageManager) -> JsProject {
        JsProject { root, manifest, package_manager }
    }
    
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
    
    pub fn package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
}

impl Project for JsProject {
    fn root(&self) -> &NormalizedPath {
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
    fn tags(&self) -> &[&'static Tag] {
        &[&JS_TAG]
    }
}