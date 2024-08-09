use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};
use ring_traits::{Detector, DetectorResult, Scope};
use ring_utils::PathTree;
use crate::{RustProjectDetector, RustScope};
use crate::cargo_loader::CargoLoader;

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

    fn cargo_loader(&self) -> &CargoLoader {
        self.project_detector.cargo_loader()
    }

    pub fn load_at(&self, path: &Path) -> anyhow::Result<Option<Rc<RustScope>>> {
        let manifest = self.cargo_loader().load(path)?;

        Ok(manifest
            .filter(|mnf| mnf.workspace.is_some())
            .map(|mnf| {
                let scope = RustScope::new(path.to_path_buf(), mnf, self.project_detector.clone());
                debug!("Found rust scope at {}", path.display());

                let project = Rc::new(scope);
                self.cache.borrow_mut().set(path, project.clone());

                project
            }))
    }

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item=anyhow::Result<Rc<RustScope>>> + 'a {
        info!("Searching rust scope from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|ancestor| self.load_at(ancestor))
            .filter_map(|result| match result {
                Ok(Some(prj)) => Some(Ok(prj)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            })
    }
}

impl Detector for RustScopeDetector {
    type Item = Rc<dyn Scope>;
    
    fn detect_from(&self, path: &Path) -> DetectorResult<Self::Item> {
        if let Some(res) = self.search_form(path).next() {
            res.map(|scp| scp as Rc<dyn Scope>).into()
        } else {
            DetectorResult::None
        }
    }
}