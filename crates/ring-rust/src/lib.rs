mod cargo_manifest;
mod constants;
mod project;
mod project_detector;
mod scope;
mod scope_detector;

use std::rc::Rc;
pub use cargo_manifest::{CargoManifest, CargoPackage, CargoWorkspace};
pub use project::RustProject;
pub use project_detector::RustProjectDetector;
use ring_traits::{Module, ProjectDetector, TaggedDetector};
pub use scope::RustScope;
pub use scope_detector::RustScopeDetector;

// Module
#[derive(Debug)]
pub struct RustModule {
    project_detector: Rc<RustProjectDetector>,
    scope_detector: Rc<RustScopeDetector>,
}

impl RustModule {
    pub fn new() -> RustModule {
        let project_detector = Rc::new(RustProjectDetector::new());

        RustModule {
            project_detector: project_detector.clone(),
            scope_detector: Rc::new(RustScopeDetector::new(project_detector))
        }
    }
}

impl Default for RustModule {
    fn default() -> Self {
        RustModule::new()
    }
}

impl Module for RustModule {
    fn project_detectors(&self) -> Vec<Rc<ProjectDetector>> {
        vec![
            self.project_detector.clone()
        ]
    }

    fn tagged_detectors(&self) -> Vec<Rc<TaggedDetector>> {
        vec![
            self.project_detector.clone(),
            self.scope_detector.clone()
        ]
    }
}