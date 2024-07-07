use std::fmt::{Display, Formatter};
use anyhow::{Context, Result};
use std::path::{Path};
use std::rc::Rc;
use semver::Version;
use tracing::{debug, trace};
use ring_project::{Project, Workspace};
use ring_utils::store::Store;
use crate::constants::{LOCKFILES, MANIFEST};
use crate::package_manifest::PackageManifest;
use crate::{JsWorkspace, PackageManager};
use crate::workspace_searcher::JsWorkspaceSearcher;

#[derive(Debug)]
pub struct JsProject {
    main_workspace: Rc<JsWorkspace>,
    package_manager: PackageManager,
    workspace_store: Store<JsWorkspace, JsWorkspaceSearcher>,
}

impl JsProject {
    pub fn new(root: &Path, package_manager: PackageManager) -> Result<JsProject> {
        let main_workspace = Rc::new(JsWorkspace::new(root)?);
        let workspace_store = Store::new_from(
            JsWorkspaceSearcher::new(&main_workspace.manifest().workspaces, root),
            &[main_workspace.clone()]
        );

        Ok(JsProject { main_workspace, package_manager, workspace_store })
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

    pub fn manifest(&self) -> &PackageManifest {
        self.main_workspace.manifest()
    }

    pub fn package_manager(&self) -> &PackageManager {
        &self.package_manager
    }
}

impl Project for JsProject {
    type Workspace = JsWorkspace;

    fn workspaces(&self) -> impl Iterator<Item = Result<Rc<Self::Workspace>>> {
        self.workspace_store.iter()
    }
}

impl Workspace for JsProject {
    fn name(&self) -> &str {
        self.main_workspace.name()
    }

    fn root(&self) -> &Path {
        self.main_workspace.root()
    }

    fn version(&self) -> Option<&Version> {
        self.main_workspace.version()
    }
}

impl Display for JsProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reference())
    }
}