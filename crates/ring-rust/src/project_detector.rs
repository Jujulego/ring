use crate::constants::MANIFEST;
use crate::{CargoManifest, RustProject};
use ring_files::ManifestLoader;
use ring_traits::{Detector, DetectAs, Project, Tagged};
use ring_utils::OptionalResult::{self, Empty, Found};
use ring_utils::PathTree;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};

#[derive(Debug)]
pub struct RustProjectDetector {
    cache: RefCell<PathTree<Rc<RustProject>>>,
    cargo_loader: ManifestLoader<CargoManifest>,
}

impl RustProjectDetector {
    pub fn new() -> RustProjectDetector {
        RustProjectDetector {
            cache: RefCell::new(PathTree::new()),
            cargo_loader: ManifestLoader::new(MANIFEST),
        }
    }

    pub(crate) fn cargo_loader(&self) -> &ManifestLoader<CargoManifest> {
        &self.cargo_loader
    }

    pub fn load_at(&self, path: &Path) -> OptionalResult<Rc<RustProject>> {
        if let Some(project) = self.cache.borrow().get(path) {
            debug!("Found rust project {} at {} (cached)", project.name(), path.display());
            return Found(project.clone());
        }

        self.cargo_loader.load(path)
            .filter(|mnf| mnf.package.is_some())
            .map(|mnf| Rc::new(RustProject::new(path.to_path_buf(), mnf)))
            .inspect(|prj| {
                debug!("Found rust project {} at {}", prj.name(), path.display());
                self.cache.borrow_mut().set(path, prj.clone());
            })
    }

    pub fn search_form(&self, path: &Path) -> OptionalResult<Rc<RustProject>> {
        info!("Searching rust project from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|anc| self.load_at(anc))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }
}

impl Default for RustProjectDetector {
    fn default() -> Self {
        RustProjectDetector::new()
    }
}

impl Detector for RustProjectDetector {
    type Item = Rc<RustProject>;

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        self.search_form(path)
    }
}

impl DetectAs<Rc<dyn Project>> for RustProjectDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Project>> {
        self.detect_from(path)
            .map(|prj| prj as Rc<dyn Project>)
    }
}

impl DetectAs<Rc<dyn Tagged>> for RustProjectDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Tagged>> {
        self.detect_from(path)
            .map(|prj| prj as Rc<dyn Tagged>)
    }
}