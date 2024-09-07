use crate::constants::rust_tag;
use crate::{CargoManifest, CargoWorkspace, RustProjectDetector};
use ring_files::PatternIterator;
use ring_traits::{ProjectIterator, Scope};
use ring_utils::{NormalizedPath, NormalizedPathBuf, Tag, Tagged};
use std::rc::Rc;
use tracing::{debug, warn};

#[derive(Debug)]
pub struct RustScope {
    root: NormalizedPathBuf,
    manifest: Rc<CargoManifest>,
    project_detector: Rc<RustProjectDetector>,
}

impl RustScope {
    pub fn new(root: NormalizedPathBuf, manifest: Rc<CargoManifest>, project_detector: Rc<RustProjectDetector>) -> RustScope {
        RustScope { root, manifest, project_detector }
    }
    
    pub fn workspace(&self) -> &CargoWorkspace {
        self.manifest.workspace.as_ref().unwrap()
    }
}

impl Scope for RustScope {
    fn root(&self) -> &NormalizedPath {
        &self.root
    }

    fn projects(&self) -> Box<ProjectIterator> {
        let projects = self.workspace().members.iter()
            .resolve(self.root())
            .inspect(|pattern| debug!("Search rust project matching {}", pattern.display()))
            .glob_search()
            .filter_map(|result| result
                .inspect_err(|err| warn!("Error while loading scope project {:#}", err))
                .ok()
            )
            .detect_at(self.project_detector.clone());

        Box::new(projects)
    }
}

impl Tagged for RustScope {
    fn tags(&self) -> Vec<Tag> {
        vec![rust_tag()]
    }
}