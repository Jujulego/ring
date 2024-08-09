use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, trace};
use ring_traits::{Detector, DetectorResult, Project, Scope};
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

impl Detector for JsScopeDetector {
    type Item = Rc<dyn Scope>;
    
    fn detect_from(&self, path: &Path) -> DetectorResult<Self::Item> {
        for project in self.project_detector.search_form(path) {
            match project {
                Ok(project) => {
                    for (package_manager, lockfile) in LOCKFILES {
                        let lockfile = project.root().join(lockfile);
                        trace!("Testing {}", lockfile.display());

                        match lockfile.try_exists().with_context(|| format!("Unable to access {}", lockfile.display())) {
                            Ok(true) => {
                                debug!("Found lockfile {}", lockfile.display());
                                debug!("Detected package manager {}", package_manager);

                                let scope = JsScope::new(project, package_manager, self.project_detector.clone());
                                return DetectorResult::Found(Rc::new(scope));
                            }
                            Ok(false) => continue,
                            Err(err) => {
                                return DetectorResult::Err(err)
                            }
                        }
                    }
                }
                Err(err) => {
                    return DetectorResult::Err(err);
                }
            }
        }

        DetectorResult::None
    }
}