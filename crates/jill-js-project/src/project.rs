use std::fmt::{Display, Formatter};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use glob::glob;
use tracing::{debug, trace};
use jill_project::Workspace;
use crate::constants::{LOCKFILES, MANIFEST};
use crate::package_manifest::PackageManifest;
use crate::PackageManager;
use crate::workspace::JsWorkspace;
use crate::workspace_iterator::WorkspaceIterator;

#[derive(Debug)]
pub struct JsProject {
    root: PathBuf,
    manifest: PackageManifest,
    package_manager: PackageManager,
}

impl JsProject {
    pub fn new(root: &Path, package_manager: PackageManager) -> Result<JsProject> {
        Ok(JsProject {
            root: root.to_path_buf(),
            manifest: PackageManifest::parse_file(&root.join(MANIFEST))?,
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

    pub fn workspaces(&self) -> WorkspaceIterator {
        return WorkspaceIterator::new(&self.manifest.workspaces, &self.root);
    }

    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }

    pub fn package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
}

impl Workspace for JsProject {
    fn name(&self) -> &str {
        &self.manifest.name
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn version(&self) -> Option<&str> {
        self.manifest.version.as_deref()
    }
}

impl Display for JsProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reference())
    }
}