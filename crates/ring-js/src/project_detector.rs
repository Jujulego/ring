use crate::constants::MANIFEST;
use crate::{JsProject, PackageManifest};
use ring_files::ManifestLoader;
use ring_traits::{Detector, DetectAs, Project, Tagged, detect_as, detect_from};
use ring_utils::OptionalResult::{self, Found};
use ring_utils::PathTree;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};
use crate::lockfile_detector::JsLockfileDetector;

#[derive(Debug)]
pub struct JsProjectDetector {
    cache: RefCell<PathTree<Rc<JsProject>>>,
    lockfile_detector: JsLockfileDetector,
    package_loader: ManifestLoader<PackageManifest>,
}

impl JsProjectDetector {
    pub fn new() -> JsProjectDetector {
        JsProjectDetector {
            cache: RefCell::new(PathTree::new()),
            lockfile_detector: JsLockfileDetector::new(),
            package_loader: ManifestLoader::new(MANIFEST),
        }
    }
}

impl Default for JsProjectDetector {
    fn default() -> Self {
        JsProjectDetector::new()
    }
}

impl Detector for JsProjectDetector {
    type Item = Rc<JsProject>;

    fn detect_at(&self, path: &Path) -> OptionalResult<Self::Item> {
        let path = if path.is_file() { path.parent().unwrap() } else { path };
        
        if let Some(project) = self.cache.borrow().get(path) {
            debug!("Found js project {} at {} (cached)", project.name(), path.display());
            return Found(project.clone());
        }

        self.package_loader.load(path)
            .and_then(|mnf|
                self.lockfile_detector.detect_from(path)
                    .result_or_default()
                    .map(|lck| (mnf, lck))
            )
            .map(|(mnf, lck)| Rc::new(JsProject::new(path.to_path_buf(), mnf, lck)))
            .inspect(|prj| {
                debug!("Found js project {} at {}", prj.name(), path.display());
                self.cache.borrow_mut().set(path, prj.clone());
            })
    }

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        info!("Searching js project from {}", path.display());
        detect_from!(self, path)
    }
}

detect_as!(JsProjectDetector, Rc<dyn Tagged>);
detect_as!(JsProjectDetector, Rc<dyn Project>);