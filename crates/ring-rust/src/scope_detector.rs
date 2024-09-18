use crate::{CargoManifest, RustProjectDetector, RustScope};
use ring_files::ManifestLoader;
use ring_traits::{detect_as, detect_from, Detect, DetectAs, Scope};
use ring_utils::OptionalResult::{self, Found};
use ring_utils::{NormalizedPath, PathTree, Tagged};
use std::cell::RefCell;
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

    pub(crate) fn cargo_loader(&self) -> &ManifestLoader<CargoManifest> {
        self.project_detector.cargo_loader()
    }
}

impl Detect for RustScopeDetector {
    type Item = Rc<RustScope>;

    fn detect_at(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        let path = if path.is_file() { path.parent().unwrap() } else { path };

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

    fn detect_from(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        info!("Searching rust scope from {}", path.display());
        detect_from!(self, path)
    }
}

detect_as!(RustScopeDetector, Rc<dyn Scope>);
detect_as!(RustScopeDetector, Rc<dyn Tagged>);