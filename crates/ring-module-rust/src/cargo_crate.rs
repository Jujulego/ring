use crate::rust_language;
use ring_core_content::Language;
use ring_core_units::Unit;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Parsed content of Cargo.toml file
#[derive(Clone, Debug, Deserialize)]
pub struct CargoManifest {
    pub package: Option<CargoPackage>,
    pub workspace: Option<CargoWorkspace>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoPackage {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoWorkspace {
    pub members: Vec<String>,
}

/// Represents a cargo crate unit
#[derive(Clone, Debug)]
pub struct CargoCrate {
    manifest: CargoManifest,
    root: PathBuf,
}

impl CargoCrate {
    /// Creates a new cargo crate
    pub fn new(manifest: CargoManifest, root: PathBuf) -> CargoCrate {
        CargoCrate { manifest, root }
    }

    /// Returns loaded Cargo.toml manifest
    pub fn manifest(&self) -> &CargoManifest {
        &self.manifest
    }

    /// Returns crate's name read from the manifest, if any.
    pub fn name(&self) -> Option<&str> {
        self.manifest.package.as_ref()
            .map(|pkg| pkg.name.as_str())
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
        self.name()
            .or_else(|| self.root.file_name().and_then(|s| s.to_str()))
    }
}