use crate::constants::LOCKFILES;
use crate::{JsProjectDetector, JsScope};
use anyhow::Context;
use ring_traits::{Detector, Project, Scope};
use ring_utils::OptionalResult::{self, Empty, Fail, Found};
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, trace};

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

    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
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
                                return Found(Rc::new(scope));
                            }
                            Ok(false) => continue,
                            Err(err) => {
                                return Fail(err)
                            }
                        }
                    }
                }
                Err(err) => {
                    return Fail(err);
                }
            }
        }

        Empty
    }
}