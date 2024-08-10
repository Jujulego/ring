use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};
use ring_traits::{Detector, OptionalResult, Project};
use ring_utils::{ManifestLoader, PathTree};
use crate::{CargoManifest, RustProject};
use crate::constants::MANIFEST;

#[derive(Debug)]
pub struct RustProjectDetector {
    cache: RefCell<PathTree<Rc<RustProject>>>,
    cargo_loader: ManifestLoader<CargoManifest>,
}

impl RustProjectDetector {
    pub fn new() -> RustProjectDetector {
        RustProjectDetector {
            cargo_loader: ManifestLoader::new(MANIFEST),
            cache: RefCell::new(PathTree::new()),
        }
    }

    pub(crate) fn cargo_loader(&self) -> &ManifestLoader<CargoManifest> {
        &self.cargo_loader
    }

    pub fn load_at(&self, path: &Path) -> OptionalResult<Rc<RustProject>> {
        self.cargo_loader.load(path)
            .filter(|mnf| mnf.package.is_some())
            .map(|mnf| Rc::new(RustProject::new(path.to_path_buf(), mnf)))
            .inspect(|prj| {
                debug!("Found rust project {} at {}", prj.name(), path.display());
                self.cache.borrow_mut().set(path, prj.clone());
            })
    }

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item=anyhow::Result<Rc<RustProject>>> + 'a {
        info!("Searching rust project from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|ancestor| self.load_at(ancestor))
            .filter_map(|result| result.into_option())
    }
}

impl Default for RustProjectDetector {
    fn default() -> Self {
        RustProjectDetector::new()
    }
}

impl Detector for RustProjectDetector {
    type Item = Rc<dyn Project>;
    
    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        if let Some(res) = self.search_form(path).next() {
            res.map(|prj| prj as Rc<dyn Project>).into()
        } else {
            OptionalResult::None
        }
    }
}