use std::path::{Path, PathBuf};
use cargo_toml::Manifest;
use ring_core_content::Language;
use ring_core_units::Unit;
use crate::rust_language;

/// Represents a cargo crate unit
#[derive(Clone, Debug)]
pub struct CargoCrate {
    manifest: Manifest,
    root: PathBuf,
}

impl CargoCrate {
    /// Creates a new cargo crate
    pub fn new(manifest: Manifest, root: PathBuf) -> CargoCrate {
        CargoCrate { manifest, root }
    }

    /// Returns loaded package.json manifest
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Returns crate's name read from the manifest, if any.
    pub fn name(&self) -> Option<&str> {
        self.manifest.package.as_ref().map(|pkg| pkg.name())
    }

    /// Returns true if the loaded crate is a workspace
    pub fn is_workspace(&self) -> bool {
        self.manifest.workspace.is_some()
    }
}

impl Unit for CargoCrate {
    /// Returns the detected kind of unit, either `"cargo:crate"` or `"cargo:workspace"`
    fn kind(&self) -> &str {
        if self.is_workspace() {
            "cargo:workspace"
        } else {
            "cargo:crate"
        }
    }

    /// Returns the given root path
    fn root(&self) -> &Path {
        &self.root
    }

    /// Returns javascript language object
    fn language(&self) -> Option<Language> {
        Some(rust_language())
    }

    /// Returns package name read from the manifest, if any.
    fn name(&self) -> Option<&str> {
        self.manifest.package.as_ref()
            .map(|pkg| pkg.name())
            .or_else(|| self.root.file_name().and_then(|s| s.to_str()))
    }
}