use crate::constants::JS_TAG;
use crate::{JsProject, JsProjectDetector, PackageManager};
use anyhow::Context;
use glob::glob;
use ring_files::PatternIterator;
use ring_traits::{Detector, Project, Scope, Tagged};
use ring_utils::Tag;
use std::path::Path;
use std::rc::Rc;
use tracing::debug;

#[derive(Debug)]
pub struct JsScope {
    root_project: Rc<JsProject>,
    project_detector: Rc<JsProjectDetector>,
}

impl JsScope {
    pub fn new(root_project: Rc<JsProject>, project_detector: Rc<JsProjectDetector>) -> JsScope {
        JsScope { root_project, project_detector }
    }

    pub fn root_project(&self) -> &Rc<JsProject> {
        &self.root_project
    }

    pub fn package_manager(&self) -> &PackageManager {
        self.root_project.package_manager()
    }
}

impl Scope for JsScope {
    fn root(&self) -> &Path {
        self.root_project.root()
    }

    fn projects<'a>(&'a self) -> Box<dyn Iterator<Item=anyhow::Result<Rc<dyn Project>>> + 'a> {
        let patterns = self.root_project.manifest().workspaces.iter();

        Box::new(patterns.relative_to(self.root())
            .inspect(|pattern| debug!("Search js project matching {pattern}"))
            .filter_map(|pattern| glob(&pattern).ok())
            .flatten()
            .map(|path| {
                path.map_err(|err| err.into())
                    .and_then(|path| path.canonicalize().with_context(|| format!("Unable to access {}", path.display())))
                    .and_then(|path| self.project_detector.detect_at(&path).into())
            })
            .filter_map(|result| match result {
                Ok(Some(prj)) => Some(Ok(prj as Rc<dyn Project>)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            }))
    }
}

impl Tagged for JsScope {
    fn tags(&self) -> &[&'static Tag] {
        &[&JS_TAG]
    }
}
