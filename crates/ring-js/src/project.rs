use crate::constants::{dev_tag, js_tag, optional_tag};
use crate::package_manifest::PackageManifest;
use crate::PackageManager;
use ring_traits::{DependencyIterator, Project};
use ring_utils::{Dependency, NormalizedPath, NormalizedPathBuf, Tag, Tagged};
use semver::Version;
use std::rc::Rc;
use crate::utils::parse_js_requirement;

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

    fn dependencies(&self) -> Box<DependencyIterator> {
        let deps = self.manifest.dependencies.iter()
            .map(|(name, req)| parse_js_requirement(req, &self.root)
                .map(|req| Dependency::new(name.to_string(), req))
            );
        
        let dev_deps = self.manifest.dev_dependencies.iter()
            .map(|(name, req)| parse_js_requirement(req, &self.root)
                .map(|req| Dependency::new(name.to_string(), req).with_tag(dev_tag()))
            );
        
        let opt_deps = self.manifest.optional_dependencies.iter()
            .map(|(name, req)| parse_js_requirement(req, &self.root)
                .map(|req| Dependency::new(name.to_string(), req).with_tag(optional_tag()))
            );
        
        Box::new(deps.chain(dev_deps).chain(opt_deps))
    }
}

impl Tagged for JsProject {
    fn tags(&self) -> Vec<Tag> {
        vec![js_tag()]
    }
}