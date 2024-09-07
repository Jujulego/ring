use crate::constants::js_tag;
use crate::package_manifest::PackageManifest;
use crate::PackageManager;
use ring_traits::Project;
use ring_utils::{NormalizedPath, NormalizedPathBuf, Tag, Tagged};
use semver::Version;
use std::rc::Rc;

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
    fn tags(&self) -> Vec<Tag> {
        vec![js_tag()]
    }
}