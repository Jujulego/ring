use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, info, trace};
use ring_traits::{Detector, OptionalResult, Project};
use ring_utils::PathTree;
use crate::constants::MANIFEST;
use crate::JsProject;

#[derive(Debug)]
pub struct JsProjectDetector {
    loaded: RefCell<PathTree<Rc<JsProject>>>,
}

impl JsProjectDetector {
    pub fn new() -> JsProjectDetector {
        JsProjectDetector {
            loaded: RefCell::new(PathTree::new())
        }
    }

    pub fn load_at(&self, path: &Path) -> anyhow::Result<Option<Rc<JsProject>>> {
        if let Some(project) = self.loaded.borrow().get(path) {
            debug!("Found js project {} at {} (cached)", project.name(), path.display());
            return Ok(Some(project.clone()));
        }

        let manifest_file = path.join(MANIFEST);

        trace!("Testing {}", manifest_file.display());
        let manifest_exists = manifest_file.try_exists()
            .with_context(|| format!("Unable to access {}", manifest_file.display()))?;

        if manifest_exists {
            let project = JsProject::new(path.to_path_buf())?;
            debug!("Found js project {} at {}", project.name(), path.display());

            let project = Rc::new(project);
            self.loaded.borrow_mut().set(path, project.clone());

            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item = anyhow::Result<Rc<JsProject>>> + 'a {
        info!("Searching js project from {}", path.display());
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

impl Default for JsProjectDetector {
    fn default() -> Self {
        JsProjectDetector::new()
    }
}

impl Detector for JsProjectDetector {
    type Item = Rc<dyn Project>;
    
    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        if let Some(res) = self.search_form(path).next() {
            res.map(|prj| prj as Rc<dyn Project>).into()
        } else {
            OptionalResult::None
        }
    }
}