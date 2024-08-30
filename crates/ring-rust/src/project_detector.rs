use crate::constants::MANIFEST;
use crate::{CargoManifest, RustProject};
use ring_files::ManifestLoader;
use ring_traits::{Detector, DetectAs, Project, Tagged, detect_as, detect_from};
use ring_utils::OptionalResult::{self, Found};
use ring_utils::{NormalizedPath, PathTree};
use std::cell::RefCell;
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
}

impl Default for RustProjectDetector {
    fn default() -> Self {
        RustProjectDetector::new()
    }
}

impl Detector for RustProjectDetector {
    type Item = Rc<RustProject>;

    fn detect_at(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        let path = if path.is_file() { path.parent().unwrap() } else { path };

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

    fn detect_from(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        info!("Searching rust project from {}", path.display());
        detect_from!(self, path)
    }
}

detect_as!(RustProjectDetector, Rc<dyn Project>);
detect_as!(RustProjectDetector, Rc<dyn Tagged>);