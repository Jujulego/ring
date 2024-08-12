use std::cell::RefCell;
use crate::constants::LOCKFILES;
use crate::{JsProjectDetector, JsScope};
use anyhow::Context;
use ring_traits::{Detector, DetectAs, Project, Scope, Tagged, detect_as};
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
}

impl Detector for JsScopeDetector {
    type Item = Rc<JsScope>;

    fn detect_at(&self, path: &Path) -> OptionalResult<Self::Item> {
        if let Some(scope) = self.cache.borrow().get(path) {
            debug!("Found js scope at {} (cached)", path.display());
            return Found(scope.clone());
        }

        self.project_detector.detect_at(path)
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
    
    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        info!("Searching js scope from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|anc| self.detect_at(anc))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }
}

detect_as!(JsScopeDetector, Rc<dyn Scope>);
detect_as!(JsScopeDetector, Rc<dyn Tagged>); 