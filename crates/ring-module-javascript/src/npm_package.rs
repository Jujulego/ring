use crate::javascript_language;
use ring_core_file::Language;
use ring_core_units::Unit;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Parsed content of package.json files
#[derive(Clone, Debug, Deserialize)]
pub struct PackageManifest {
    pub name: Option<String>,
    #[serde(default)]
    pub workspaces: Vec<String>,
}

/// Represents a npm package unit
#[derive(Clone, Debug)]
pub struct NpmPackage {
    manifest: PackageManifest,
    root: PathBuf,
}

impl NpmPackage {
    /// Create a new npm package
    pub fn new(manifest: PackageManifest, root: PathBuf) -> NpmPackage {
        NpmPackage { manifest, root }
    }

    /// Returns loaded package.json manifest
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
}

impl Unit for NpmPackage {
    /// Returns the detected kind of unit.
    /// Either `"npm:package"` or `"npm:workspace"`
    fn kind(&self) -> &str {
        if self.manifest.workspaces.is_empty() {
            "npm:package"
        } else {
            "npm:workspace"
        }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn language(&self) -> Option<Language> {
        Some(javascript_language())
    }

    fn name(&self) -> Option<&str> {
        self.manifest.name.as_deref()
    }
}
