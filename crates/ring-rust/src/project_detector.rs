use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};
use ring_traits::{Project, ProjectDetector};
use ring_utils::PathTree;
use crate::RustProject;
use crate::cargo_loader::CargoLoader;

#[derive(Debug)]
pub struct RustProjectDetector {
    cache: RefCell<PathTree<Rc<RustProject>>>,
    cargo_loader: CargoLoader,
}

impl RustProjectDetector {
    pub fn new() -> RustProjectDetector {
        RustProjectDetector {
            cargo_loader: CargoLoader::new(),
            cache: RefCell::new(PathTree::new()),
        }
    }

    pub(crate) fn cargo_loader(&self) -> &CargoLoader {
        &self.cargo_loader
    }

    pub fn load_at(&self, path: &Path) -> anyhow::Result<Option<Rc<RustProject>>> {
        let manifest = self.cargo_loader.load(path)?;

        Ok(manifest
            .and_then(|mnf| mnf.package.clone())
            .map(|pkg| {
                let project = RustProject::new(path.to_path_buf(), pkg);
                debug!("Found rust project {} at {}", project.name(), path.display());

                let project = Rc::new(project);
                self.cache.borrow_mut().set(path, project.clone());

                project
            }))
    }

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item=anyhow::Result<Rc<RustProject>>> + 'a {
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