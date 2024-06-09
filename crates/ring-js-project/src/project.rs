use anyhow::{Context, Result};
use std::path::Path;
use glob::glob;
use tracing::{debug, trace};
use crate::constants::{LOCKFILES, MANIFEST};
use crate::PackageManager;
use crate::workspace::JsWorkspace;

#[derive(Debug)]
pub struct JsProject {
    main_workspace: JsWorkspace,
    package_manager: PackageManager,
}

impl JsProject {
    pub fn new(root: &Path, package_manager: PackageManager) -> Result<JsProject> {
        Ok(JsProject {
            main_workspace: JsWorkspace::new(root)?,
            package_manager
        })
    }

    pub fn search_from(path: &Path) -> Result<Option<JsProject>> {
        let mut root = if path.is_file() { path.parent().unwrap() } else { path };
        let mut manifest = None;

        loop {
            trace!("Testing {}", root.display());

            for (package_manager, lockfile) in LOCKFILES {
                let lockfile = root.join(lockfile);

                if lockfile.try_exists().context(format!("Unable to access {}", lockfile.display()))? {
                    debug!("Found lockfile {}", lockfile.display());
                    debug!("Detected package manager {}", package_manager);

                    return Ok(Some(JsProject::new(root, package_manager)?));
                }
            }

            {
                let file = root.join(MANIFEST);

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
            debug!("No package manager detected, uses {} by default", PackageManager::default());
            Ok(Some(JsProject::new(root, PackageManager::default())?))
        } else {
            Ok(None)
        }
    }

    pub fn list_workspaces(&self) -> Result<()> {
        let patterns = self.main_workspace.get_manifest().workspaces.iter()
            .map(|pattern| self.get_root().join(pattern).join("package.json"));

        for pattern in patterns {
            let pattern = pattern.to_str().unwrap();
            
            #[cfg(windows)]
            let pattern = &pattern[4..];
            
            trace!("List manifests matching pattern {pattern}");

            for manifest in glob(pattern)? {
                debug!("Found manifest {}", manifest?.display());
            }
        }

        Ok(())
    }

    pub fn main_workspace(&self) -> &JsWorkspace {
        &self.main_workspace
    }

    pub fn get_package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
    
    pub fn get_root(&self) -> &Path {
        &self.main_workspace.get_root()
    }
}