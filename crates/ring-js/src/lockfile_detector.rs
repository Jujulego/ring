use std::cell::RefCell;
use anyhow::anyhow;
use tracing::{debug, info, trace};
use ring_traits::{detect_from, Detect};
use ring_utils::{NormalizedPath, OptionalResult, PathTree};
use ring_utils::OptionalResult::{Empty, Fail, Found};
use crate::constants::PACKAGE_MANAGERS;
use crate::PackageManager;

#[derive(Debug)]
pub struct JsLockfileDetector {
    cache: RefCell<PathTree<PackageManager>>
}

impl JsLockfileDetector {
    pub fn new() -> JsLockfileDetector {
        JsLockfileDetector {
            cache: RefCell::new(PathTree::new())
        }
    }
}

impl Default for JsLockfileDetector {
    fn default() -> Self {
        JsLockfileDetector::new()
    }
}

impl Detect for JsLockfileDetector {
    type Item = PackageManager;

    fn detect_at(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        let path = if path.is_file() { path.parent().unwrap() } else { path };

        if let Some(&pm) = self.cache.borrow().get(path) {
            debug!("Found {} lockfile at {} (cached)", pm, path.display());
            return Found(pm);
        }

        for package_manager in PACKAGE_MANAGERS {
            let lockfile = path.join(package_manager.lockfile());
            trace!("Testing {}", lockfile.display());

            match lockfile.try_exists() {
                Ok(true) => {
                    debug!("Found {} lockfile at {}", package_manager, path.display());
                    self.cache.borrow_mut().set(path, package_manager);

                    return Found(package_manager);
                }
                Ok(false) => continue,
                Err(err) => {
                    return Fail(anyhow!(err).context(format!("Unable to access {}", lockfile.display())));
                }
            }
        }

        Empty
    }

    fn detect_from(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        info!("Searching js lockfile from {}", path.display());
        detect_from!(self, path)
    }
}