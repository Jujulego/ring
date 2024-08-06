use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, info, trace};
use ring_traits::{Project, ProjectDetector};
use ring_utils::PathTree;
use crate::constants::MANIFEST;
use crate::RustProject;

#[derive(Debug)]
pub struct RustProjectDetector {
    loaded: RefCell<PathTree<Rc<RustProject>>>,
}

impl RustProjectDetector {
    pub fn new() -> RustProjectDetector {
        RustProjectDetector {
            loaded: RefCell::new(PathTree::new())
        }
    }
    
    pub fn load_at(&self, path: &Path) -> anyhow::Result<Option<Rc<RustProject>>> {
        if let Some(project) = self.loaded.borrow().get(path) {
            debug!("Found rust project {} at {} (cached)", project.name(), path.display());
            return Ok(Some(project.clone()))
        }

        let manifest_file = path.join(MANIFEST);
        
        trace!("Testing {}", manifest_file.display());
        let manifest_exists = manifest_file.try_exists()
            .with_context(|| format!("Unable to access {}", manifest_file.display()))?;

        if manifest_exists {
            let project = RustProject::new(path.to_path_buf())?;
            debug!("Found rust project {} at {}", project.name(), path.display());

            let project = Rc::new(project);
            self.loaded.borrow_mut().set(path, project.clone());

            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item = anyhow::Result<Rc<RustProject>>> + 'a {
        info!("Searching rust project from {}", path.display());
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

impl Default for RustProjectDetector {
    fn default() -> Self {
        RustProjectDetector::new()
    }
}

impl ProjectDetector for RustProjectDetector {
    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn Project>>> {
        if let Some(res) = self.search_form(path).next() {
            res.map(|prj| Some(prj as Rc<dyn Project>))
        } else {
            Ok(None)
        }
    }
}