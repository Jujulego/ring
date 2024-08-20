mod constants;
mod lockfile_detector;
mod package_manager;
mod package_manifest;
mod project;
mod project_detector;
mod scope;
mod scope_detector;

pub use package_manager::PackageManager;
pub use package_manifest::PackageManifest;
pub use project::JsProject;
pub use project_detector::JsProjectDetector;
pub use scope::JsScope;
pub use scope_detector::JsScopeDetector;
use std::rc::Rc;
use ring_traits::{Module, TaggedDetector};

// Module
#[derive(Debug)]
pub struct JsModule {
    project_detector: Rc<JsProjectDetector>,
    scope_detector: Rc<JsScopeDetector>,
}

impl JsModule {
    pub fn new() -> JsModule {
        let project_detector = Rc::new(JsProjectDetector::new());
        
        JsModule {
            project_detector: project_detector.clone(),
            scope_detector: Rc::new(JsScopeDetector::new(project_detector))
        }
    }
}

impl Default for JsModule {
    fn default() -> Self {
        JsModule::new()
    }
}

impl Module for JsModule {
    fn tagged_detectors(&self) -> Vec<Rc<TaggedDetector>> {
        vec![
            self.project_detector.clone(),
            self.scope_detector.clone()
        ]
    }
}