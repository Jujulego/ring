use crate::{CargoManifest, RustProjectDetector, RustScope};
use ring_files::ManifestLoader;
use ring_traits::OptionalResult::{Empty, Found};
use ring_traits::{Detector, OptionalResult, Scope};
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

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item=anyhow::Result<Rc<RustScope>>> + 'a {
        info!("Searching rust scope from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|ancestor| self.load_at(ancestor))
            .filter_map(|result| result.into_option())
    }
}

impl Detector for RustScopeDetector {
    type Item = Rc<dyn Scope>;

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        if let Some(res) = self.search_form(path).next() {
            res.map(|scp| scp as Rc<dyn Scope>).into()
        } else {
            Empty
        }
    }
}