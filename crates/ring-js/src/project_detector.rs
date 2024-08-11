use crate::constants::MANIFEST;
use crate::{JsProject, PackageManifest};
use ring_files::ManifestLoader;
use ring_traits::OptionalResult::{Empty, Found};
use ring_traits::{Detector, OptionalResult, Project};
use ring_utils::PathTree;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, info};

#[derive(Debug)]
pub struct JsProjectDetector {
    cache: RefCell<PathTree<Rc<JsProject>>>,
    package_loader: ManifestLoader<PackageManifest>,
}

impl JsProjectDetector {
    pub fn new() -> JsProjectDetector {
        JsProjectDetector {
            cache: RefCell::new(PathTree::new()),
            package_loader: ManifestLoader::new(MANIFEST),
        }
    }

    pub fn load_at(&self, path: &Path) -> OptionalResult<Rc<JsProject>> {
        if let Some(project) = self.cache.borrow().get(path) {
            debug!("Found js project {} at {} (cached)", project.name(), path.display());
            return Found(project.clone());
        }

        self.package_loader.load(path)
            .map(|mnf| Rc::new(JsProject::new(path.to_path_buf(), mnf)))
            .inspect(|prj| {
                debug!("Found js project {} at {}", prj.name(), path.display());
                self.cache.borrow_mut().set(path, prj.clone());
            })
    }

    pub fn search_form<'a>(&'a self, path: &'a Path) -> impl Iterator<Item=anyhow::Result<Rc<JsProject>>> + 'a {
        info!("Searching js project from {}", path.display());
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        path.ancestors()
            .map(|ancestor| self.load_at(ancestor))
            .filter_map(|result| result.into_option())
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
            Empty
        }
    }
}