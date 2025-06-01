use crate::javascript_language;
use ring_core_content::Language;
use ring_core_units::Unit;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Parsed content of package.json file
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
    #[inline]
    pub fn new(manifest: PackageManifest, root: PathBuf) -> NpmPackage {
        NpmPackage { manifest, root }
    }

    /// Returns loaded package.json manifest
    #[inline]
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }

    /// Returns package's name read from the manifest, if any.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.manifest.name.as_deref()
    }

    /// Returns true if the loaded package is a workspace
    #[inline]
    pub fn is_workspace(&self) -> bool {
        self.manifest.workspaces.is_empty()
    }
}

impl Unit for NpmPackage {
    /// Returns the detected kind of unit, either `"npm:package"` or `"npm:workspace"`
    #[inline]
    fn kind(&self) -> &str {
        if self.is_workspace() {
            "npm:package"
        } else {
            "npm:workspace"
        }
    }

    /// Returns the given root path
    #[inline]
    fn root(&self) -> &Path {
        &self.root
    }

    /// Returns javascript language object
    #[inline]
    fn language(&self) -> Option<Language> {
        Some(javascript_language())
    }

    /// Returns package name read from the manifest, if any.
    #[inline]
    fn name(&self) -> Option<&str> {
        self.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_package_kind() {
        let package = NpmPackage::new(
            PackageManifest { name: Some("test".into()), workspaces: vec![] },
            "/test".into()
        );

        assert_eq!(package.kind(), "npm:package");
    }

    #[test]
    fn it_should_return_workspace_kind() {
        let package = NpmPackage::new(
            PackageManifest { name: Some("test".into()), workspaces: vec!["packages/*".into()] },
            "/test".into()
        );

        assert_eq!(package.kind(), "npm:workspace");
    }

    #[test]
    fn it_should_return_javascript_language() {
        let package = NpmPackage::new(
            PackageManifest { name: Some("test".into()), workspaces: vec!["packages/*".into()] },
            "/test".into()
        );

        assert_eq!(package.language(), Some(javascript_language()));
    }

    #[test]
    fn it_should_return_manifest_name() {
        let package = NpmPackage::new(
            PackageManifest { name: Some("test".into()), workspaces: vec!["packages/*".into()] },
            "/test".into()
        );

        assert_eq!(package.name(), Some("test"));
    }
}