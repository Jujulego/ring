use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, trace};
use ring_traits::{Project, ProjectDetector};
use ring_utils::PathTree;
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
}

impl Default for JsProjectDetector {
    fn default() -> Self {
        JsProjectDetector::new()
    }
}

impl ProjectDetector for JsProjectDetector {
    type Project = JsProject;

    fn search_from(&self, path: &Path) -> anyhow::Result<Option<Rc<Self::Project>>> {
        debug!("Searching js project from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        for ancestor in path.ancestors() {
            if let Some(project) = self.loaded.borrow().get(ancestor) {
                debug!("Found js project {} at {} (cached)", project.name(), ancestor.display());
                return Ok(Some(project.clone()));
            }

            let manifest_file = ancestor.join("package.json");

            trace!("Testing {}", manifest_file.display());
            let manifest_exists = manifest_file.try_exists()
                .with_context(|| format!("Unable to access {}", manifest_file.display()))?;

            if manifest_exists {
                let project = JsProject::new(ancestor.to_path_buf())?;
                debug!("Found js project {} at {}", project.name(), ancestor.display());

                let project = Rc::new(project);
                self.loaded.borrow_mut().set(ancestor, project.clone());

                return Ok(Some(project));
            }
        }

        Ok(None)
    }
}