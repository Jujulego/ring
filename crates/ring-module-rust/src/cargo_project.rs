use ring_core_file::{FileContent, QualifyPath};
use ring_core_language::{DetectLanguage, Language};
use ring_module_toml::toml_language;
use std::ffi::OsStr;
use std::path::Path;
use tracing::instrument;

#[derive(Clone, Debug, Default)]
pub struct CargoProjectDetector {}

impl CargoProjectDetector {
    /// Creates a new instance of CargoManifestDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a Cargo manifest
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_manifest("Cargo.toml"));
    /// assert!(!detector.is_manifest("src"));
    /// ```
    #[inline]
    pub fn is_manifest<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_manifest(path.as_ref())
    }

    fn _is_manifest(&self, path: &Path) -> bool {
        path.is_file() && path.file_name().and_then(OsStr::to_str) == Some("Cargo.toml")
    }

    /// Checks if given path is a Cargo lockfile
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_rust::CargoProjectDetector;
    ///
    /// let detector = CargoProjectDetector::new();
    /// assert!(detector.is_lockfile("../../Cargo.lock"));
    /// assert!(!detector.is_lockfile("src"));
    /// ```
    #[inline]
    pub fn is_lockfile<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_lockfile(path.as_ref())
    }

    fn _is_lockfile(&self, path: &Path) -> bool {
        path.is_file() && path.file_name().and_then(OsStr::to_str) == Some("Cargo.lock")
    }
}

impl DetectLanguage for CargoProjectDetector {
    #[instrument(name = "cargo-manifest.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_manifest(path) || self._is_lockfile(path) {
            Some(toml_language())
        } else {
            None
        }
    }
}

impl QualifyPath for CargoProjectDetector {
    #[instrument(name = "cargo-manifest.qualify-path", skip_all)]
    fn qualify_content(&self, path: &Path) -> Option<FileContent> {
        if self._is_manifest(path) {
            Some(FileContent::Manifest)
        } else if self._is_lockfile(path) {
            Some(FileContent::Lockfile)
        } else {
            None
        }
    }
}