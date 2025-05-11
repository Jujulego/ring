use crate::rust_language;
use ring_core_content::Language;
use ring_core_units::Unit;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Parsed content of Cargo.toml file
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CargoManifest {
    pub package: Option<CargoPackage>,
    pub workspace: Option<CargoWorkspace>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CargoPackage {
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
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
    #[inline]
    pub fn new(manifest: CargoManifest, root: PathBuf) -> CargoCrate {
        CargoCrate { manifest, root }
    }

    /// Returns loaded Cargo.toml manifest
    #[inline]
    pub fn manifest(&self) -> &CargoManifest {
        &self.manifest
    }

    /// Returns crate's name read from the manifest, if any.
    pub fn name(&self) -> Option<&str> {
        self.manifest.package.as_ref()
            .map(|pkg| pkg.name.as_str())
            .or_else(|| self.root.file_name().and_then(|s| s.to_str()))
    }

    /// Returns true if the loaded crate is a workspace
    #[inline]
    pub fn is_workspace(&self) -> bool {
        self.manifest.workspace.is_some()
    }
}

impl Unit for CargoCrate {
    /// Returns the detected kind of unit, either `"cargo:crate"` or `"cargo:workspace"`
    #[inline]
    fn kind(&self) -> &str {
        if self.is_workspace() {
            "cargo:workspace"
        } else {
            "cargo:crate"
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
        Some(rust_language())
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
    fn it_should_mark_crate_as_package() {
        let manifest = CargoManifest {
            package: Some(CargoPackage {
                name: "package".to_string(),
            }),
            ..Default::default()
        };

        let crt = CargoCrate::new(manifest, PathBuf::from("/test"));

        assert!(!crt.is_workspace());
        assert_eq!(crt.kind(), "cargo:crate");
    }

    #[test]
    fn it_should_mark_crate_as_workspace() {
        let manifest = CargoManifest {
            workspace: Some(Default::default()),
            ..Default::default()
        };

        let crt = CargoCrate::new(manifest, PathBuf::from("/test"));

        assert!(crt.is_workspace());
        assert_eq!(crt.kind(), "cargo:workspace");
    }

    #[test]
    fn name_should_return_package_name_from_manifest() {
        let manifest = CargoManifest {
            package: Some(CargoPackage {
                name: "package".to_string(),
            }),
            ..Default::default()
        };

        let crt = CargoCrate::new(manifest, PathBuf::from("/test"));

        assert_eq!(crt.name(), Some("package"));
    }

    #[test]
    fn name_should_return_folder_name() {
        let manifest = CargoManifest {
            workspace: Some(Default::default()),
            ..Default::default()
        };

        let crt = CargoCrate::new(manifest, PathBuf::from("/test"));

        assert_eq!(crt.name(), Some("test"));
    }
}