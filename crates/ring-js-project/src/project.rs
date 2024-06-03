use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, trace};
use crate::constants::{LOCKFILES, MANIFEST};
use crate::PackageManager;

pub struct JsProject {
    package_manager: PackageManager,
    root: PathBuf,
}

impl JsProject {
    pub fn search_from(path: &Path) -> Result<Option<JsProject>> {
        let mut root = if path.is_file() { path.parent().unwrap() } else { path };
        let mut manifest = None;

        loop {
            trace!("Testing {}", root.display());

            for (package_manager, lockfile) in LOCKFILES {
                let lockfile = root.to_path_buf().join(lockfile);

                if lockfile.try_exists().context(format!("Unable to access {}", lockfile.display()))? {
                    debug!("Found lockfile {}", lockfile.display());
                    debug!("Detected package manager {}", package_manager);
                    
                    return Ok(Some(JsProject {
                        package_manager,
                        root: root.to_path_buf()
                    }));
                }
            }

            {
                let file = root.to_path_buf().join(MANIFEST);

                if file.try_exists().context(format!("Unable to access {}", file.display()))? {
                    debug!("Found manifest {}", file.display());
                    
                    manifest = Some(file);
                }
            }

            // Move up
            if let Some(parent) = root.parent() {
                root = parent;
            } else {
                break;
            }
        }

        if let Some(root) = manifest.as_deref().and_then(Path::parent) {
            debug!("Uses package manager {} by default", PackageManager::NPM);
            
            Ok(Some(JsProject {
                root: root.to_path_buf(),
                package_manager: PackageManager::NPM,
            }))
        } else {
            Ok(None)
        }
    }
    
    pub fn get_package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
    
    pub fn get_root(&self) -> &Path {
        &self.root
    }
}