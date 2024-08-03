use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::debug;
use ring_traits::{Project, ScopeDetector};
use crate::{JsProjectDetector, JsScope};
use crate::constants::LOCKFILES;

#[derive(Debug)]
pub struct JsScopeDetector {
    project_detector: Rc<JsProjectDetector>,
}

impl JsScopeDetector {
    pub fn new(project_detector: Rc<JsProjectDetector>) -> JsScopeDetector {
        JsScopeDetector { project_detector }
    }
}

impl ScopeDetector for JsScopeDetector {
    type Scope = JsScope;

    fn detect_from(&self, path: &Path) -> anyhow::Result<Option<Rc<Self::Scope>>> {
        for project in self.project_detector.search_form(path) {
            let project = project?;

            for (package_manager, lockfile) in LOCKFILES {
                let lockfile = project.root().join(lockfile);

                if lockfile.try_exists().with_context(|| format!("Unable to access {}", lockfile.display()))? {
                    debug!("Found lockfile {}", lockfile.display());
                    debug!("Detected package manager {}", package_manager);

                    let scope = JsScope::new(project, package_manager, self.project_detector.clone());
                    return Ok(Some(Rc::new(scope)));
                }
            }
        }

        Ok(None)
    }
}