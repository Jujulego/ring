use crate::{CargoManifest, RustProjectDetector, RustScope};
use ring_files::ManifestLoader;
use ring_traits::{Detector, DetectAs, Scope, Tagged};
use ring_utils::OptionalResult::{self, Empty, Found};
use ring_utils::PathTree;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};

#[derive(Debug)]
pub struct RustScopeDetector {
    cache: RefCell<PathTree<Rc<RustScope>>>,
    project_detector: Rc<RustProjectDetector>,
}

impl RustScopeDetector {
    pub fn new(project_detector: Rc<RustProjectDetector>) -> RustScopeDetector {
        RustScopeDetector {
            cache: RefCell::new(PathTree::new()),
            project_detector,
        }
    }

    fn cargo_loader(&self) -> &ManifestLoader<CargoManifest> {
        self.project_detector.cargo_loader()
    }

    pub fn load_at(&self, path: &Path) -> OptionalResult<Rc<RustScope>> {
        if let Some(scope) = self.cache.borrow().get(path) {
            debug!("Found rust scope at {} (cached)", path.display());
            return Found(scope.clone());
        }

        self.cargo_loader().load(path)
            .filter(|mnf| mnf.workspace.is_some())
            .map(|mnf| Rc::new(RustScope::new(path.to_path_buf(), mnf, self.project_detector.clone())))
            .inspect(|scp| {
                debug!("Found rust scope at {}", path.display());
                self.cache.borrow_mut().set(path, scp.clone());
            })
    }

    pub fn search_form(&self, path: &Path) -> OptionalResult<Rc<RustScope>> {
        info!("Searching rust scope from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|anc| self.load_at(anc))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }
}

impl Detector for RustScopeDetector {
    type Item = Rc<RustScope>;

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        self.search_form(path)
    }
}

impl DetectAs<Rc<dyn Scope>> for RustScopeDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Scope>> {
        self.detect_from(path)
            .map(|scp| scp as Rc<dyn Scope>)
    }
}

impl DetectAs<Rc<dyn Tagged>> for RustScopeDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Tagged>> {
        self.detect_from(path)
            .map(|scp| scp as Rc<dyn Tagged>)
    }
}