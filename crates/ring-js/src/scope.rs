use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use glob::glob;
use tracing::debug;
use ring_traits::{Project, Scope, Tagged};
use crate::{JsProject, JsProjectDetector, PackageManager};

#[derive(Debug)]
pub struct JsScope {
    root_project: Rc<JsProject>,
    package_manager: PackageManager,
    project_detector: Rc<JsProjectDetector>,
}

impl JsScope {
    pub fn new(root_project: Rc<JsProject>, package_manager: PackageManager, project_detector: Rc<JsProjectDetector>) -> JsScope {
        JsScope { root_project, package_manager, project_detector }
    }

    pub fn root_project(&self) -> &Rc<JsProject> {
        &self.root_project
    }

    pub fn package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
}

impl Scope for JsScope {
    fn root(&self) -> &Path {
        self.root_project.root()
    }

    fn projects<'a>(&'a self) -> Box<dyn Iterator<Item=anyhow::Result<Rc<dyn Project>>> + 'a> {
        let patterns = self.root_project.manifest().workspaces.iter()
            .map(|pattern| self.root().join(pattern));

        Box::new(patterns
            .inspect(|pattern| debug!("Search js project matching {}", pattern.display()))
            .filter_map(|pattern| {
                #[cfg(windows)]
                { glob(&pattern.to_str().unwrap()[4..]).ok() }

                #[cfg(not(windows))]
                { glob(pattern.to_str().unwrap()).ok() }
            })
            .flatten()
            .map(|path| {
                path.map_err(|err| err.into())
                    .and_then(|path| path.canonicalize().with_context(|| format!("Unable to access {}", path.display())))
                    .and_then(|path| self.project_detector.load_at(&path).into_result())
            })
            .filter_map(|result| match result {
                Ok(Some(prj)) => Some(Ok(prj as Rc<dyn Project>)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            }))
    }
}

impl Tagged for JsScope {
    fn tags(&self) -> &[&'static str] {
        &["js"]
    }
}