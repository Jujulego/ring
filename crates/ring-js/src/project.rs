use std::path::{Path, PathBuf};
use semver::Version;
use ring_traits::Project;
use crate::package_manifest::PackageManifest;

#[derive(Debug)]
pub struct JsProject {
    root: PathBuf,
    manifest: PackageManifest,
}

impl JsProject {
    pub fn new(root: PathBuf) -> anyhow::Result<JsProject> {
        let manifest = PackageManifest::parse_file(&root.join("package.json"))?;
        
        Ok(JsProject {
            root,
            manifest,
        })
    }
}

impl Project for JsProject {
    fn root(&self) -> &Path {
        &self.root
    }

    fn name(&self) -> &str {
        &self.manifest.name
    }

    fn version(&self) -> Option<&Version> {
        self.manifest.version.as_ref()
    }

    fn tag(&self) -> &[&str] {
        &["js"]
    }
}