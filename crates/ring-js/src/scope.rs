use crate::constants::JS_TAG;
use crate::{JsProject, JsProjectDetector, PackageManager};
use ring_files::PatternIterator;
use ring_traits::{Project, ProjectIterator, Scope, Tagged};
use ring_utils::{NormalizedPath, Tag};
use std::rc::Rc;
use tracing::{debug, warn};

#[derive(Debug)]
pub struct JsScope {
    root_project: Rc<JsProject>,
    project_detector: Rc<JsProjectDetector>,
}

impl JsScope {
    pub fn new(root_project: Rc<JsProject>, project_detector: Rc<JsProjectDetector>) -> JsScope {
        JsScope {
            root_project,
            project_detector,
        }
    }

    pub fn root_project(&self) -> &Rc<JsProject> {
        &self.root_project
    }

    pub fn package_manager(&self) -> &PackageManager {
        self.root_project.package_manager()
    }
}

impl Scope for JsScope {
    fn root(&self) -> &NormalizedPath {
        self.root_project.root()
    }

    fn projects(&self) -> Box<ProjectIterator> {
        let projects = self.root_project.manifest().workspaces.iter()
            .resolve(self.root())
            .inspect(|pattern| debug!("Search js project matching {}", pattern.display()))
            .glob_search()
            .filter_map(|result| result
                .inspect_err(|err| warn!("Error while loading scope project {:#}", err))
                .ok()
            )
            .detect_at(self.project_detector.clone());

        Box::new(projects)
    }
}

impl Tagged for JsScope {
    fn tags(&self) -> &[&'static Tag] {
        &[&JS_TAG]
    }
}
