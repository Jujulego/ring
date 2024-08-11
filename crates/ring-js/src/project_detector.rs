use crate::constants::MANIFEST;
use crate::{JsProject, PackageManifest};
use ring_files::ManifestLoader;
use ring_traits::{Detector, DetectAs, Project, Tagged};
use ring_utils::OptionalResult::{self, Empty, Found};
use ring_utils::PathTree;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};

#[derive(Debug)]
pub struct JsProjectDetector {
    cache: RefCell<PathTree<Rc<JsProject>>>,
    package_loader: ManifestLoader<PackageManifest>,
}

impl JsProjectDetector {
    pub fn new() -> JsProjectDetector {
        JsProjectDetector {
            cache: RefCell::new(PathTree::new()),
            package_loader: ManifestLoader::new(MANIFEST),
        }
    }

    pub fn load_at(&self, path: &Path) -> OptionalResult<Rc<JsProject>> {
        if let Some(project) = self.cache.borrow().get(path) {
            debug!("Found js project {} at {} (cached)", project.name(), path.display());
            return Found(project.clone());
        }

        self.package_loader.load(path)
            .map(|mnf| Rc::new(JsProject::new(path.to_path_buf(), mnf)))
            .inspect(|prj| {
                debug!("Found js project {} at {}", prj.name(), path.display());
                self.cache.borrow_mut().set(path, prj.clone());
            })
    }

    pub fn search_form(&self, path: &Path) -> OptionalResult<Rc<JsProject>> {
        info!("Searching js project from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|ancestor| self.load_at(ancestor))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }
}

impl Default for JsProjectDetector {
    fn default() -> Self {
        JsProjectDetector::new()
    }
}

impl Detector for JsProjectDetector {
    type Item = Rc<JsProject>;

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        self.search_form(path)
    }
}

impl DetectAs<Rc<dyn Project>> for JsProjectDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Project>> {
        self.detect_from(path)
            .map(|prj| prj as Rc<dyn Project>)
    }
}

impl DetectAs<Rc<dyn Tagged>> for JsProjectDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Tagged>> {
        self.detect_from(path)
            .map(|prj| prj as Rc<dyn Tagged>)
    }
}