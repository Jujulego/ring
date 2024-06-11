use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, trace};
use jill_project::Workspace;
use crate::constants::{LOCKFILES, MANIFEST};
use crate::package_manifest::PackageManifest;
use crate::{PackageManager, workspace_store};
use crate::workspace_store::WorkspaceStore;

#[derive(Debug)]
pub struct JsProject {
    root: PathBuf,
    manifest: PackageManifest,
    package_manager: PackageManager,
    workspace_store: RefCell<WorkspaceStore>,
}

impl JsProject {
    pub fn new(root: &Path, package_manager: PackageManager) -> Result<JsProject> {
        let manifest = PackageManifest::parse_file(&root.join(MANIFEST))?;
        let root = root.to_path_buf();
        let workspace_store = RefCell::new(WorkspaceStore::new(&manifest.workspaces, &root));

        Ok(JsProject { root, manifest, package_manager, workspace_store })
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

    pub fn workspaces(&self) -> workspace_store::Iter {
        workspace_store::Iter::new(&self.workspace_store)
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