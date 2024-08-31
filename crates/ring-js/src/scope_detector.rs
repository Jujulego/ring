use std::cell::RefCell;
use crate::{JsProjectDetector, JsScope};
use ring_traits::{Detect, DetectAs, Scope, Tagged, detect_as, detect_from};
use ring_utils::OptionalResult::{self, Found};
use std::rc::Rc;
use tracing::{debug, info};
use ring_utils::{NormalizedPath, PathTree};

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

impl Detect for JsScopeDetector {
    type Item = Rc<JsScope>;

    fn detect_at(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        if let Some(scope) = self.cache.borrow().get(path) {
            debug!("Found js scope at {} (cached)", path.display());
            return Found(scope.clone());
        }

        self.project_detector.detect_at(path)
            .filter(|prj| !prj.manifest().workspaces.is_empty())
            .map(|prj| Rc::new(JsScope::new(prj, self.project_detector.clone())))
            .inspect(|scp| {
                debug!("Found js scope at {}", path.display());
                self.cache.borrow_mut().set(path, scp.clone())
            })
    }
    
    fn detect_from(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        info!("Searching js scope from {}", path.display());
        detect_from!(self, path)
    }
}

detect_as!(JsScopeDetector, Rc<dyn Scope>);
detect_as!(JsScopeDetector, Rc<dyn Tagged>); 