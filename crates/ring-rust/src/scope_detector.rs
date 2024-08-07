use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, info, trace};
use ring_traits::{Scope, ScopeDetector};
use ring_utils::PathTree;
use crate::{CargoManifest, RustProjectDetector, RustScope};
use crate::constants::MANIFEST;

#[derive(Debug)]
pub struct RustScopeDetector {
    loaded: RefCell<PathTree<Rc<RustScope>>>,
    project_detector: Rc<RustProjectDetector>,
}

impl RustScopeDetector {
    pub fn new(project_detector: Rc<RustProjectDetector>) -> RustScopeDetector {
        RustScopeDetector {
            loaded: RefCell::new(PathTree::new()),
            project_detector
        }
    }

    pub fn load_at(&self, path: &Path) -> anyhow::Result<Option<Rc<RustScope>>> {
        if let Some(scope) = self.loaded.borrow().get(path) {
            debug!("Found rust scope at {} (cached)", path.display());
            return Ok(Some(scope.clone()));
        }

        let manifest_file = path.join(MANIFEST);

        trace!("Testing {}", manifest_file.display());
        let manifest_exists = manifest_file.try_exists()
            .with_context(|| format!("Unable to access {}", manifest_file.display()))?;

        if manifest_exists {
            let manifest = CargoManifest::parse_file(&manifest_file)?;

            Ok(manifest.workspace
                .map(|wks| {
                    let scope = RustScope::new(path.to_path_buf(), wks, self.project_detector.clone());
                    debug!("Found rust scope at {}", path.display());

                    let project = Rc::new(scope);
                    self.loaded.borrow_mut().set(path, project.clone());

                    project
                }))
        } else {
            Ok(None)
        }
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

impl ScopeDetector for RustScopeDetector {
    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn Scope>>> {
        if let Some(res) = self.search_form(path).next() {
            res.map(|scp| Some(scp as Rc<dyn Scope>))
        } else {
            Ok(None)
        }
    }
}