use std::cell::RefCell;
use crate::constants::LOCKFILES;
use crate::{JsProjectDetector, JsScope};
use anyhow::Context;
use ring_traits::{Detector, DetectAs, Project, Scope, Tagged};
use ring_utils::OptionalResult::{self, Empty, Fail, Found};
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info, trace};
use ring_utils::PathTree;

#[derive(Debug)]
pub struct JsScopeDetector {
    cache: RefCell<PathTree<Rc<JsScope>>>,
    project_detector: Rc<JsProjectDetector>,
}

impl JsScopeDetector {
    pub fn new(project_detector: Rc<JsProjectDetector>) -> JsScopeDetector {
        JsScopeDetector {
            cache: RefCell::new(PathTree::new()),
            project_detector
        }
    }

    pub fn load_at(&self, path: &Path) -> OptionalResult<Rc<JsScope>> {
        if let Some(scope) = self.cache.borrow().get(path) {
            debug!("Found js scope at {} (cached)", path.display());
            return Found(scope.clone());
        }

        self.project_detector.load_at(path)
            .and_then(|prj| {
                for (package_manager, lockfile) in LOCKFILES {
                    let lockfile = prj.root().join(lockfile);
                    trace!("Testing {}", lockfile.display());

                    match lockfile.try_exists().with_context(|| format!("Unable to access {}", lockfile.display())) {
                        Ok(true) => {
                            debug!("Found lockfile {}", lockfile.display());
                            debug!("Detected package manager {}", package_manager);
                            debug!("Found js scope at {}", path.display());

                            let scope = JsScope::new(prj, package_manager, self.project_detector.clone());
                            return Found(Rc::new(scope));
                        }
                        Ok(false) => continue,
                        Err(err) => {
                            return Fail(err)
                        }
                    }
                }

                Empty
            })
            .inspect(|scp| self.cache.borrow_mut().set(path, scp.clone()))
    }

    pub fn search_from(&self, path: &Path) -> OptionalResult<Rc<JsScope>> {
        info!("Searching js scope from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|anc| self.load_at(anc))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }
}

impl Detector for JsScopeDetector {
    type Item = Rc<JsScope>;

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        self.search_from(path)
    }
}

impl DetectAs<Rc<dyn Scope>> for JsScopeDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Scope>> {
        self.detect_from(path)
            .map(|scp| scp as Rc<dyn Scope>)
    }
}

impl DetectAs<Rc<dyn Tagged>> for JsScopeDetector {
    fn detect_from_as(&self, path: &Path) -> OptionalResult<Rc<dyn Tagged>> {
        self.detect_from(path)
            .map(|scp| scp as Rc<dyn Tagged>)
    }
}